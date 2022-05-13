package main

import (
	"bufio"
	"encoding/hex"
	"fmt"
	"math"
	"os"
)

type Packet struct {
	version    int
	typeID     int
	literal    int
	subPackets []Packet
}

func versionSum(p Packet) int {
	sum := 0
	for _, child := range p.subPackets {
		sum += versionSum(child)
	}
	return sum + p.version
}

func expr(p Packet) int {
	// 8 choices of typeID
	switch p.typeID {
	case 0:
		// Packets with type ID 0 are sum packets
		result := 0
		for _, child := range p.subPackets {
			result += expr(child)
		}
		return result
	case 1:
		// Packets with type ID 1 are product packets
		result := 1
		for _, child := range p.subPackets {
			result *= expr(child)
		}
		return result
	case 2:
		// Packets with type ID 2 are minimum packets
		result := math.MaxInt64
		for _, child := range p.subPackets {
			r := expr(child)
			if r < result {
				result = r
			}
		}
		return result
	case 3:
		// Packets with type ID 3 are maximum packets
		result := math.MinInt64
		for _, child := range p.subPackets {
			r := expr(child)
			if r > result {
				result = r
			}
		}
		return result
	case 4:
		// Packets with type ID 4 are literal packets
		return p.literal
	case 5:
		// Packets with type ID 5 are greater than packets
		first := expr(p.subPackets[0])
		second := expr(p.subPackets[1])

		if first > second {
			return 1
		} else {
			return 0
		}
	case 6:
		// Packets with type ID 6 are less than packets
		first := expr(p.subPackets[0])
		second := expr(p.subPackets[1])

		if first < second {
			return 1
		} else {
			return 0
		}
	case 7:
		// Packets with type ID 7 are equal to packets
		first := expr(p.subPackets[0])
		second := expr(p.subPackets[1])

		if first == second {
			return 1
		} else {
			return 0
		}
	default:
		panic("invalid packetID")
	}
}

func (p Packet) String() string {
	return p.StringerHelper(0)
}

func (p Packet) StringerHelper(indentation int) string {
	var s string
	for i := 0; i < indentation; i++ {
		s += " "
	}

	if p.typeID == 4 {
		s += "Literal{ "
	} else {
		s += "Operator{ "
	}

	s += fmt.Sprintf("version: %d, ", p.version)
	if p.typeID == 4 {
		s += fmt.Sprintf("value: %d }", p.literal)
	} else {
		// print sub packets
		s += "[\n"
		for _, child := range p.subPackets {
			s += fmt.Sprintf("%s,\n", child.StringerHelper(indentation+2))
		}
		for i := 0; i < indentation; i++ {
			s += " "
		}
		s += "]}"
	}

	return s
}

type BitStream struct {
	bits []int
	pos  int
}

func (bs *BitStream) Read(n int) []int {
	if n > len(bs.bits)-bs.pos {
		panic("not enough bits")
	}

	result := bs.bits[bs.pos : bs.pos+n]
	bs.pos += n
	return result
}

func (bs *BitStream) ReadInt(n int) int {
	if n > 64 {
		panic("n must be <= 64")
	}

	// read the bits
	bits := bs.Read(n)

	// convert the bits into a single integer
	return bitsToInt(bits)
}

func bitsToInt(bits []int) int {
	result := 0
	for _, bit := range bits {
		result = (result << 1) | bit
	}
	return result
}

func readPacket(bs *BitStream) Packet {
	// Every packet begins with a standard header: the first three bits encode the packet version, and the next three bits encode the packet type ID.
	version := bs.ReadInt(3)
	typeID := bs.ReadInt(3)
	var subPackets []Packet

	if typeID == 4 {
		// Packets with type ID 4 represent a literal value. Literal value packets encode a single binary number.
		// To do this, the binary number is padded with leading zeroes until its length is a multiple of four bits, and then it is broken into groups of four bits.
		// Each group is prefixed by a 1 bit except the last group, which is prefixed by a 0 bit.
		bits := make([]int, 0)

		for {
			// read the first bit
			prefix := bs.Read(1)[0]

			// read the 4 bit group
			group := bs.Read(4)
			bits = append(bits, group...)

			// if the prefix is 0, we're done
			if prefix == 0 {
				break
			}
		}

		// convert the bits into a single integer
		return Packet{version, typeID, bitsToInt(bits), nil}
	} else {
		// Every other type of packet (any packet with a type ID other than 4) represent an operator that performs some calculation on one or more sub-packets contained within.
		// Right now, the specific operations aren't important; focus on parsing the hierarchy of sub-packets.
		//
		// An operator packet contains one or more packets. To indicate which subsequent binary data represents its sub-packets, an operator packet can use one of two modes indicated by the bit immediately after the packet header;
		// this is called the length type ID:
		lengthTypeId := bs.ReadInt(1)

		if lengthTypeId == 0 {
			// If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
			length := bs.ReadInt(15)

			// create a bitstream for the sub-packets
			subBs := &BitStream{bs.Read(length), 0}

			// read the sub-packets
			subPackets = make([]Packet, 0)
			for {
				// read the next packet
				p := readPacket(subBs)
				subPackets = append(subPackets, p)

				// if the bitstream is empty, we're done
				if subBs.pos == len(subBs.bits) {
					break
				}
			}
		} else {
			// If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
			packets := bs.ReadInt(11)

			subPackets = make([]Packet, packets)
			for i := 0; i < packets; i++ {
				subPackets[i] = readPacket(bs)
			}
		}

		return Packet{version, typeID, 0, subPackets}
	}
}

func main() {
	// read input file
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// the input file is lines of hexadecimal numbers
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		bytes, err := hex.DecodeString(scanner.Text())
		if err != nil {
			panic(err)
		}

		// convert the bytes into a list of integers 0 or 1
		bits := make([]int, 0, len(bytes)*8)
		for _, b := range bytes {
			for i := 7; i >= 0; i-- {
				bits = append(bits, int((b>>i)&1))
			}
		}

		packet := readPacket(&BitStream{bits, 0})

		// fmt.Println(packet)

		// for part 1 we need to calculate the version number sum
		fmt.Println(versionSum(packet))

		// for part 2 we need to evalulate the expression
		fmt.Println(expr(packet))
	}
}
