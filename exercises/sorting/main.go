package main

import "fmt"

func main() {
	ints := []int{2, 3, 4, 1, 6, 5}
	res := mergeSort(ints)
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
	if len(nums) <= 1 {
		return nums
	}

	mid := len(nums) / 2
	left := mergeSort(nums[:mid])
	right := mergeSort(nums[mid:])

	return merge(left, right)
}

func merge(left, right []int) []int {
	sorted := []int{}

	i := 0
	j := 0

	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			sorted = append(sorted, left[i])
			i++
		} else {
			sorted = append(sorted, right[j])
			j++
		}
	}

	if i < len(left) {
		sorted = append(sorted, left[i:]...)
	}
	if j < len(right) {
		sorted = append(sorted, right[j:]...)
	}

	return sorted
}

