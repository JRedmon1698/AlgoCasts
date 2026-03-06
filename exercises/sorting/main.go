package main

import "fmt"

func main() {
	ints := []int{2, 3, 4, 1, 6}
	res := insertionSort(ints)
	fmt.Println(res)
}

func insertionSort(nums []int) []int {
	for i := 1; i < len(nums); i++ {
		j := i - 1

		for j >= 0 && nums[j+1] < nums[j] {
			tmp := nums[j+1]
			nums[j+1] = nums[j]
			nums[j] = tmp
			j--
		}
	}

	return nums
}

func mergeSort(nums []int) []int {

}

func merge(left, right []int) []int {

}
