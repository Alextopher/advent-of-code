// PriorityQueue stolen from https://pkg.go.dev/container/heap#example-package-PriorityQueue
// This example demonstrates a priority queue built using the heap interface.
package main

import (
	"container/heap"
)

// An Item is something we manage in a priority queue.
type Item struct {
	value struct {
		x, y int
	} // The value of the item; arbitrary.
	priority int // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].priority > pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	item := old[0]
	old[0] = nil    // avoid memory leak
	item.index = -1 // for safety
	*pq = old[1:]
	return item
}

func (pq *PriorityQueue) SetPriority(value struct{ x, y int }, priority int) {
	for _, item := range *pq {
		if item.value.x == value.x && item.value.y == value.y {
			pq.update(item, value, priority)
			return
		}
	}
}

// update modifies the priority and value of an Item in the queue.
func (pq *PriorityQueue) update(item *Item, value struct{ x, y int }, priority int) {
	item.value = value
	item.priority = priority
	heap.Fix(pq, item.index)
}
