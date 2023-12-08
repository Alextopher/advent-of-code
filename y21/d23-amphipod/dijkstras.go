package main

import (
	"container/heap"
	"fmt"
)

type HeapItem[T comparable] struct {
	value T

	priority int // g_cost + h_cost
	h_cost   int // heuristic cost
	g_cost   int // cost to move from start to this node

	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type MinHeap[T comparable] []*HeapItem[T]

func (h MinHeap[T]) Len() int { return len(h) }

func (h MinHeap[T]) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return (h[i].priority) < h[j].priority
}

func (h MinHeap[T]) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
	h[i].index = i
	h[j].index = j
}

func (h *MinHeap[T]) Push(x any) {
	n := len(*h)
	item := x.(*HeapItem[T])
	item.index = n
	*h = append(*h, item)
}

func (h *MinHeap[T]) Pop() any {
	old := *h
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*h = old[0 : n-1]
	return item
}

func (h *MinHeap[T]) Find(x T) *HeapItem[T] {
	for _, item := range *h {
		if item.value == x {
			return item
		}
	}
	return nil
}

// update modifies the priority and value of an Item in the queue.
func (h *MinHeap[T]) update(item *HeapItem[T], value T, h_cost, g_cost int) {
	item.value = value
	item.h_cost = h_cost
	item.g_cost = g_cost
	item.priority = g_cost + h_cost
	heap.Fix(h, item.index)
}

// Dijkstra's algorithm
type Dijkstra[T comparable] interface {
	Neighbors(state T) ([]T, []int)
	Estimate(state T, goal T) int
}

func DijkstraSearch[T comparable](start T, goal T, graph Dijkstra[T]) {
	// frontier ← priority queue containing start only
	queue := MinHeap[T]{}
	heap.Push(&queue, &HeapItem[T]{value: start, priority: 0})

	// expanded ← empty set
	expanded := make(map[T]struct{})

	// do
	for {
		// if frontier is empty then return failure
		if queue.Len() == 0 {
			fmt.Println("No solution found")
			return
		}

		// node ← frontier.pop()
		node := heap.Pop(&queue).(*HeapItem[T])

		// if node is a goal state then
		if node.value == goal {
			fmt.Println(node.value, node.priority)
			return
		}

		// expanded.add(node)
		expanded[node.value] = struct{}{}

		// for each of node's neighbors n do
		neighbors, costs := graph.Neighbors(node.value)
		for i := 0; i < len(neighbors); i++ {
			neighbor := neighbors[i]
			g_cost := costs[i]

			// estimate the cost of reaching the end
			h_cost := graph.Estimate(neighbor, goal)

			priority := g_cost + node.g_cost + h_cost

			_, inExpanded := expanded[neighbor]
			v := queue.Find(neighbor)

			if !inExpanded && v == nil {
				// if n is not in expanded and not in frontier then frontier.add(n)
				heap.Push(&queue, &HeapItem[T]{value: neighbor, h_cost: h_cost, g_cost: g_cost + node.g_cost, priority: priority})
			} else if v != nil && v.priority > priority {
				// else if n is in frontier with higher cost replace existing node with n
				queue.update(v, neighbor, h_cost, g_cost+node.g_cost)
			}
		}
	}
}
