# Data Structure and Algorithms



## Sorting

### Bubble Sorting

#### Rust

```rust
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![4, 5, 3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(vec![1, 2, 3, 4, 5], v);
    }
}
```

```rust
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        let mut sorted = true;
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![4, 5, 3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(vec![1, 2, 3, 4, 5], v);
    }
}
```



#### Java

```java
package com.jason.datastructor.sort;

/**
 * @Description:
 * @author: 贾森
 * @date: 2024年04月23日 上午11:46
 */
public class BubbleSort {
    public static void main(String[] args) {
        Integer[] intArray = {64, 34, 25, 12, 22, 11, 90};
        Double[] doubleArray = {3.14, 2.718, 1.618, 0.577, 1.732};
        String[] stringArray = {"banana", "apple", "orange", "grape", "pineapple"};

        System.out.println("排序前的整型数组：");
        printArray(intArray);
        System.out.println("\n排序前的双精度数组：");
        printArray(doubleArray);
        System.out.println("\n排序前的字符串数组：");
        printArray(stringArray);

        bubbleSort(intArray);
        bubbleSort(doubleArray);
        bubbleSort(stringArray);

        System.out.println("\n排序后的整型数组：");
        printArray(intArray);
        System.out.println("\n排序后的双精度数组：");
        printArray(doubleArray);
        System.out.println("\n排序后的字符串数组：");
        printArray(stringArray);
    }
    public  static <T extends Comparable<T>> void bubbleSort(T[] array){
        int n = array.length;
        for (int i = 0;i<n-1;i++){
            for (int j=0;j<n-1-i;j++){
                if (array[j].compareTo(array[j+1])<0){
                    T temp = array[j];
                    array[j]=array[j+1];
                    array[j+1] = temp;
                }
            }
        }
    }
    // 打印数组
    public static <T> void printArray(T[] array) {
        for (T element : array) {
            System.out.print(element + " ");
        }
        System.out.println();
    }
}


```



#### Golang



```go
package main

import "fmt"

func main() {
	arr := []int{4, 5, 3, 2, 1}
	fmt.Println("排序前的数组：", arr)
	bubbleSort(arr)
	fmt.Println("排序后的数组：", arr)
}

func bubbleSort(arr []int) {
	n := len(arr)
	for i := 0; i < n-1; i++ {
		for j := 0; j < n-1-i; j++ {
			if arr[j] < arr[j+1] {
				temp := arr[j]
				arr[j] = arr[j+1]
				arr[j+1] = temp
			}
		}
	}
}
```



使用泛型

```go
package main

import "fmt"

// 定义一个接口用于实现比较
type Comparable interface {
	CompareTo(other interface{}) int
}
type Integer int

func (a Integer) CompareTo(other interface{}) int {
	b := other.(Integer)
	if a < b {
		return -1
	} else if a > b {
		return 1
	}
	return 0
}

func BubbleSort[T Comparable](arr []T) {
	n := len(arr)
	for i := 0; i < n-1; i++ {
		for j := 0; j < n-i-1; j++ {
			if arr[j].CompareTo(arr[j+1]) < 0 {
				// 交换arr[j]和arr[j+1]
				arr[j], arr[j+1] = arr[j+1], arr[j]
			}
		}
	}
}
func main() {
	arr := []Integer{4, 5, 2, 3, 1}
	fmt.Println("排序前的数组：", arr)
	BubbleSort(arr)
	fmt.Println("排序后的数组：", arr)
}

```



#### Python

```python
def bubble_sort(arr):
    n = len(arr)
    for i in range(0,n-1):
        for j in range(0,n-1-i):
            if (arr[j]>arr[j+1]):
                arr[j],arr[j+1] = arr[j+1],arr[j]

arr=[64, 34, 25, 12, 22, 11, 90]
print("排序前的数组:", arr)
bubble_sort(arr)
print("排序后的数组:", arr)
```



#### Javascript



```javascript
function bubbleSort(arr) {
  var n = arr.length;
  for (var i = 0; i < n - 1; i++) {
    for (var j = 0; j < n - 1 - i; j++) {
      if (arr[j] > arr[j + 1]) {
        var tmp = arr[j];
        arr[j] = arr[j + 1];
        arr[j + 1] = tmp;
      }
    }
  }
}

var arr = [64, 34, 25, 12, 22, 11, 90];
console.log("排序前的数组", arr);
bubbleSort(arr);
console.log("排序后的数组", arr);
```



#### TypeScript

```ts
function bubbleSort<T>(arr: T[]) {
  const n = arr.length;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n - 1 - i; j++) {
      if (arr[j] > arr[j + 1]) {
        let tmp = arr[j];
        arr[j] = arr[j + 1];
        arr[j + 1] = tmp;
      }
    }
  }
}

const arr = [64, 34, 25, 12, 22, 11, 90];
console.log("排序前的数组", arr);
bubbleSort(arr);
console.log("排序后的数组", arr);

```

#### WebAssembly

**编写排序算法的 C 代码**：

```c
#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE
void bubbleSort(int arr[], int n) {
    for (int i = 0; i < n-1; i++) {
        for (int j = 0; j < n-i-1; j++) {
            if (arr[j] > arr[j+1]) {
                // 交换 arr[j] 和 arr[j+1]
                int temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
}
```

**使用 Emscripten 编译为 Wasm 模块**：

你需要安装 Emscripten SDK，并使用 emcc 编译器将 C 代码编译为 Wasm 模块。

```css
emcc -O3 -s WASM=1 -s EXPORTED_FUNCTIONS="['_bubbleSort']" bubble.c -o bubble.js
```

这将生成一个 `bubble.wasm` 文件和一个 `bubble.js` 文件，其中 `bubble.js` 包含了一些辅助 JavaScript 代码以加载和运行 Wasm 模块。

**在 JavaScript 中调用 Wasm 模块**：

```javascript

const imports = {};
fetch('bubble.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, imports))
    .then(results => {
        const { instance } = results;
        const arr = new Int32Array([64, 34, 25, 12, 22, 11, 90]);
        const n = arr.length;
        
        console.log("排序前的数组:", arr);
        
        // 调用 Wasm 模块中的排序函数
        instance.exports.bubbleSort(arr, n);
        
        console.log("排序后的数组:", arr);
    });
```



####  Assembly

```assembly
section .data
    arr dd 64, 34, 25, 12, 22, 11, 90
    n equ ($ - arr) / 4  ; 计算数组元素个数

section .text
    global _start

_bubbleSort:
    mov ecx, n
    dec ecx             ; 设置循环次数
    mov esi, arr        ; 设置数组地址
outer_loop:
    xor edx, edx        ; 初始化内部循环索引
    mov ebx, ecx        ; 复制外部循环计数器
inner_loop:
    mov eax, [esi + edx * 4]  ; 加载当前元素到 eax
    cmp eax, [esi + edx * 4 + 4]  ; 与下一个元素比较
    jle not_swap       ; 如果当前元素 <= 下一个元素，跳过交换
    mov ebx, [esi + edx * 4 + 4]  ; 加载下一个元素到 ebx
    mov [esi + edx * 4 + 4], eax  ; 将当前元素存储到下一个位置
    mov [esi + edx * 4], ebx      ; 将下一个元素存储到当前位置
not_swap:
    inc edx            ; 更新内部循环索引
    cmp edx, ecx       ; 检查内部循环是否完成
    jl inner_loop      ; 如果内部循环未完成，继续
    dec ecx            ; 更新外部循环计数器
    jnz outer_loop     ; 如果外部循环未完成，继续
    ret

_start:
    ; 调用冒泡排序函数
    call _bubbleSort

    ; 输出排序后的数组
    mov ecx, n
    mov esi, arr
print_loop:
    mov eax, [esi]
    call print_num
    add esi, 4
    loop print_loop

    ; 退出程序
    mov eax, 1
    xor ebx, ebx
    int 0x80

print_num:
    ; 将整数转换为字符串并输出
    push eax
    call print_int
    mov eax, 0xA   ; 换行符
    call print_char
    pop eax
    ret

print_int:
    ; 递归输出整数
    cmp eax, 0
    jge print_int_positive
    mov eax, 0x2D   ; 负号 -
    call print_char
    neg eax
print_int_positive:
    xor ebx, ebx
    mov ecx, 10
    div ecx
    test edx, edx
    jz print_digit
    call print_int
print_digit:
    add eax, '0'
    mov [esp], eax
    call print_char
    ret

print_char:
    ; 输出单个字符
    mov edx, esp
    mov ecx, 1
    mov ebx, 1
    mov eax, 4
    int 0x80
    ret

```

这段代码实现了一个冒泡排序算法，使用 x86 汇编语言编写。在 _bubbleSort 函数中，它会对 arr 数组进行排序。然后在 _start 标签处调用 _bubbleSort 函数，并输出排序后的数组。



### Divide and ConquerSorting with Merge Sort

归并排序（Merge Sort）是一种基于分治思想的经典排序算法，它的主要思想是将待排序的数组分为两个子数组，然后递归地对子数组进行排序，最后将排好序的子数组合并为一个有序数组。

归并排序的基本思想可以概括为以下几个步骤：

1. **分解**：将待排序的数组分解为两个子数组，每个子数组包含原数组的一半元素。如果数组长度为1或者0，则直接返回，因为长度为1的数组是有序的。
2. **解决**：对分解得到的两个子数组分别进行归并排序，递归调用归并排序函数。
3. **合并**：将两个已经排好序的子数组合并为一个有序数组。这一步需要使用一个额外的辅助数组，将两个子数组的元素按顺序合并到辅助数组中，并最终将辅助数组的内容复制回原数组。

归并排序是一种稳定的排序算法，其时间复杂度为 O(n log n)，其中 n 是待排序数组的长度。虽然归并排序的时间复杂度较低，但是它需要额外的空间来存储辅助数组，因此在空间复杂度上不如其他原地排序算法（如快速排序）。然而，归并排序在实践中仍然被广泛应用，特别是在需要稳定排序且对空间复杂度要求不严格的情况下。

Merge Sort is a classic sorting algorithm based on the divide-and-conquer approach. Its main idea is to divide the array to be sorted into two subarrays, recursively sort the subarrays, and then merge the sorted subarrays into one sorted array.

The basic steps of Merge Sort can be summarized as follows:

1. **Divide**: Divide the array to be sorted into two subarrays, each containing half of the elements of the original array. If the length of the array is 1 or 0, simply return it since an array of length 1 is already sorted.
2. **Conquer**: Recursively apply Merge Sort to each of the two subarrays.
3. **Merge**: Merge the two sorted subarrays into one sorted array. This step requires the use of an additional auxiliary array to merge the elements of the two subarrays in sorted order, and finally copy the content of the auxiliary array back to the original array.

Merge Sort is a stable sorting algorithm with a time complexity of O(n log n), where n is the length of the array to be sorted. Although Merge Sort has a higher time complexity than some other sorting algorithms, it is widely used in practice, especially when stable sorting is required and space complexity is not a major concern.

#### Rust

```rust
use std::fmt::Debug;

/// bubble sort
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        let mut sorted = true;
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}
/// merge sort
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half
    // sort the right half
    // bring the sorted halfs together

    if v.len() <= 1 {
        return v;
    }
    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // bring them together again add whichever is lowest the front of a or the front of b
    //
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();
    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 5, 3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(vec![1, 2, 3, 4, 5], v);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);
        println!("{:?}", v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}

```



#### Java

```java
package com.jason.datastructor.sort;

/**
 * @Description:
 * @author: 贾森
 * @date: 2024年04月24日 下午3:17
 */
public class MergeSort {
    public static void main(String[] args) {
        int[] nums = new int[] { 9, 8, 7, 6, 5, 4, 3, 2, 10 };
        int[] newNums = mergeSort(nums, 0, nums.length - 1);
        for (int x : newNums) {
            System.out.println(x);
        }

    }
    public static int[] mergeSort(int[] nums,int l,int h){
        if (l == h)
            return new int[] { nums[l] };

        int mid = l + (h - l) / 2;
        int[] leftArr = mergeSort(nums, l, mid); //左有序数组
        int[] rightArr = mergeSort(nums, mid + 1, h); //右有序数组
        int[] newNum = new int[leftArr.length + rightArr.length]; //新有序数组

        int m = 0, i = 0, j = 0;
        while (i < leftArr.length && j < rightArr.length) {
            newNum[m++] = leftArr[i] <= rightArr[j] ? leftArr[i++] : rightArr[j++];
        }
        while (i < leftArr.length)
            newNum[m++] = leftArr[i++];
        while (j < rightArr.length)
            newNum[m++] = rightArr[j++];
        return newNum;
    }
}

```



```java
package com.jason.datastructor.sort;

/**
 * @Description:
 * @author: 贾森
 * @date: 2024年04月26日 下午2:21
 */
public class MergeSort1 {
    public static void mergeSortFunction(int[] array,int[] tempArray,int begin,int end){
        if (begin < end){
            int mid = begin + ((end-begin)>>1);
            mergeSortFunction(array,tempArray,begin,mid);
            mergeSortFunction(array,tempArray,mid+1,end);
            merge(array,tempArray,begin,mid,end);
        }
    }
    public static void merge(int[] array,int[] tempArray,int begin,int mid,int end){
        int leftPos = begin;
        int rightPos= mid+1;
        int tempArrayPos= begin;
        while(leftPos<=mid&&rightPos<=end){
            if (array[leftPos]<array[rightPos]){
                tempArray[tempArrayPos++] = array[leftPos++];
            } else {
                tempArray[tempArrayPos++] = array[rightPos++];
            }
        }
        while(leftPos<=mid){
            tempArray[tempArrayPos++] = array[leftPos++];
        }
        while(rightPos<=end){
            tempArray[tempArrayPos++] = array[rightPos++];
        }
        System.out.println("--------------------tempArray");
        for (int x : tempArray){
            System.out.print(x+",");
        }
        System.out.println("");
        System.out.println("--------------------tempArray");

        for (int i = begin;i<=end;i++){
            array[i] = tempArray[i];
        }


        System.out.println("--------------------array");
        for (int x : array){
            System.out.print(x+",");
        }
        System.out.println("");
        System.out.println("--------------------array");

    }
    public static void main(String[] args) {
        int[] nums = new int[] { 9, 8, 7, 6, 5, 4, 3, 2, 10 };
        int[] tempArray = new int[nums.length];
        mergeSortFunction(nums, tempArray,0, nums.length - 1);
        for (int x : nums) {
            System.out.println(x);
        }
    }
}

```



#### Golang

```go
package main

import "fmt"

func main() {
	arr := []int{4, 6, 1, 8, 11, 13, 3}
	mergeSort(arr)
	fmt.Println("排序后的数组：", arr)

}

func mergeSort(s []int) []int {
	len := len(s)
	if len == 1 {
		return s //最后切割只剩下一个元素
	}
	m := len / 2
	leftS := mergeSort(s[:m])
	rightS := mergeSort(s[m:])

	return merge(leftS, rightS)
}

// 把两个有序的切片合并成一个有序的切片
func merge(l []int, r []int) []int {
	lLen := len(l)
	rLen := len(r)
	res := make([]int, 0)

	lIndex, rIndex := 0, 0 //两个切片的下标，插入一个数，下标加一
	for lIndex < lLen && rIndex < rLen {
		if l[lIndex] > r[rIndex] {
			res = append(res, r[rIndex])
			rIndex++
		} else {
			res = append(res, l[lIndex])
			lIndex++
		}
	}
	if lIndex < lLen { //左边的还有剩余元素
		res = append(res, l[lIndex:]...)
	}
	if rIndex < rLen {
		res = append(res, r[rIndex:]...)
	}
	fmt.Println("res：", res)
	return res
}

```



#### Python

```py
def merge_sort(arr):
    if len(arr) > 1:
        mid = len(arr)//2
        left_half = arr[:mid]
        right_half = arr[mid:]
        # 递归的进行对左半部分和右半部分进行归并排序
        merge_sort(left_half)
        merge_sort(right_half)

        i = j = k = 0

        # 合并两个有序的数组

        while i<len(left_half) and j<len(right_half):
            if left_half[i] < right_half[j]:
                arr[k] = left_half[i]
                i += 1
            else:
                arr[k] = right_half[j]
                j += 1
            k += 1

        while i<len(left_half):
            arr[k] = left_half[i]
            i += 1
            k += 1

        while j<len(right_half):
            arr[k] = right_half[j]
            j += 1
            k += 1
def main():
    arr = [4, 6, 1, 8, 11, 13, 3]
    print("排序前的数组：", arr)
    merge_sort(arr)
    print("排序后的数组", arr)

if __name__  == "__main__":
    main()
```



#### Javascript

```javascript
function mergeSort(arr) {
  if (arr.length <= 1) {
    return arr;
  }
  const mid = Math.floor(arr.length / 2);
  const leftHalf = arr.slice(0, mid);
  const rightHalf = arr.slice(mid);

  return merge(mergeSort(leftHalf), mergeSort(rightHalf));
}
function merge(leftArr, rightArr) {
  let result = [];
  let leftIndex = 0;
  let rightIndex = 0;

  while (leftIndex < leftArr.length && rightIndex < rightArr.length) {
    if (leftArr[leftIndex] < rightArr[rightIndex]) {
      result.push(leftArr[leftIndex]);
      leftIndex++;
    } else {
      result.push(rightArr[rightIndex]);
      rightIndex++;
    }
  }

  return result
    .concat(leftArr.slice(leftIndex))
    .concat(rightArr.slice(rightIndex));
}
const arr = [4, 6, 1, 8, 11, 13, 3];
console.log("排序前的数组:", arr);

const sortedArr = mergeSort(arr);
console.log("排序后的数组:", sortedArr);
```



#### TypeScript

```ty
function mergeSort(arr: number[]): number[] {
    if (arr.length <= 1) {
        return arr;
    }

    const mid = Math.floor(arr.length / 2);
    const leftHalf = arr.slice(0, mid);
    const rightHalf = arr.slice(mid);

    return merge(mergeSort(leftHalf), mergeSort(rightHalf));
}

function merge(leftArr: number[], rightArr: number[]): number[] {
    let result: number[] = [];
    let leftIndex = 0;
    let rightIndex = 0;

    while (leftIndex < leftArr.length && rightIndex < rightArr.length) {
        if (leftArr[leftIndex] < rightArr[rightIndex]) {
            result.push(leftArr[leftIndex]);
            leftIndex++;
        } else {
            result.push(rightArr[rightIndex]);
            rightIndex++;
        }
    }

    return result.concat(leftArr.slice(leftIndex)).concat(rightArr.slice(rightIndex));
}

const arr = [4, 6, 1, 8, 11, 13, 3];
console.log("排序前的数组:", arr);

const sortedArr = mergeSort(arr);
console.log("排序后的数组:", sortedArr);
```

#### WebAssembly

首先，让我们编写一个 C 语言的归并排序算法，保存为 `merge_sort.c` 文件：

```c
#include <stdlib.h>

void merge(int arr[], int left, int mid, int right) {
    int n1 = mid - left + 1;
    int n2 = right - mid;

    int leftArr[n1], rightArr[n2];

    for (int i = 0; i < n1; i++) {
        leftArr[i] = arr[left + i];
    }
    for (int j = 0; j < n2; j++) {
        rightArr[j] = arr[mid + 1 + j];
    }

    int i = 0, j = 0, k = left;

    while (i < n1 && j < n2) {
        if (leftArr[i] <= rightArr[j]) {
            arr[k++] = leftArr[i++];
        } else {
            arr[k++] = rightArr[j++];
        }
    }

    while (i < n1) {
        arr[k++] = leftArr[i++];
    }

    while (j < n2) {
        arr[k++] = rightArr[j++];
    }
}

void mergeSort(int arr[], int left, int right) {
    if (left < right) {
        int mid = left + (right - left) / 2;

        mergeSort(arr, left, mid);
        mergeSort(arr, mid + 1, right);

        merge(arr, left, mid, right);
    }
}

```

接下来，我们将使用 Emscripten 工具链将该 C 代码编译为 WebAssembly 模块。假设你已经安装了 Emscripten 并配置好了环境变量，然后使用以下命令将其编译为 WebAssembly 模块：

```shell
emcc merge_sort.c -o merge_sort.js -s EXPORTED_FUNCTIONS='["_mergeSort"]' -s EXTRA_EXPORTED_RUNTIME_METHODS='["cwrap"]'
```

这将生成两个文件：`merge_sort.wasm` 和 `merge_sort.js`。其中，`merge_sort.js` 包含了 JavaScript 代码，可以用来加载和调用 `merge_sort.wasm` 中的函数。

最后，在 JavaScript 中使用这些文件：

```javascript
const fs = require('fs');
const { promisify } = require('util');
const { wasmBinary } = require('./merge_sort');

const readFileAsync = promisify(fs.readFile);

async function mergeSort(arr) {
    const { instance } = await WebAssembly.instantiate(wasmBinary);
    const mergeSortFunc = instance.exports._mergeSort;

    const memory = new Uint32Array(instance.exports.memory.buffer);

    const arrPointer = mergeSortFunc.length / 4;
    const len = arr.length;
    const pointer = arrPointer + 1;

    memory[arrPointer] = len;

    for (let i = 0; i < len; i++) {
        memory[pointer + i] = arr[i];
    }

    mergeSortFunc(pointer);

    const sortedArr = [];
    for (let i = 0; i < len; i++) {
        sortedArr.push(memory[pointer + i]);
    }

    return sortedArr;
}

async function main() {
    const arr = [4, 6, 1, 8, 11, 13, 3];
    console.log("排序前的数组:", arr);

    const sortedArr = await mergeSort(arr);
    console.log("排序后的数组:", sortedArr);
}

main();

```



#### Assembly

```assembly
section .data
    array db 64, 25, 12, 22, 11, 45, 77, 98, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
    array_size equ $ - array
    sorted_array times array_size db 0

section .text
    global _start

_start:
    mov ecx, array_size        ; 数组长度
    mov ebx, 1                 ; 缩进（递归深度）初始化为1
    jmp mergeSort              ; 调用归并排序

mergeSort:
    cmp ecx, 1                 ; 检查数组长度是否为1
    jle return                 ; 如果是，返回

    mov eax, ecx               ; 复制数组长度
    shr eax, 1                 ; 右移一位，相当于除以2
    push eax                   ; 将数组长度的一半压入栈
    push ebx                   ; 将缩进（递归深度）压入栈
    inc ebx                    ; 递归深度加1
    call mergeSort             ; 递归调用归并排序
    pop ebx                    ; 弹出递归深度
    pop eax                    ; 弹出数组长度的一半

    add esi, eax               ; 设置左侧子数组的结束位置
    mov edi, esi               ; 设置右侧子数组的开始位置
    mov edx, ecx               ; 复制数组长度
    sub edx, eax               ; 计算右侧子数组的长度

    push edx                   ; 将右侧子数组的长度压入栈
    push esi                   ; 将左侧子数组的结束位置压入栈
    mov esi, array             ; 设置左侧子数组的开始位置
    call merge                 ; 调用 merge 函数
    pop esi                    ; 弹出左侧子数组的结束位置
    pop edx                    ; 弹出右侧子数组的长度

    add edi, edx               ; 设置右侧子数组的结束位置
    mov esi, edi               ; 设置左侧子数组的开始位置

    push ecx                   ; 将数组长度压入栈
    push edi                   ; 将右侧子数组的结束位置压入栈
    call merge                 ; 调用 merge 函数
    pop edi                    ; 弹出右侧子数组的结束位置
    pop ecx                    ; 弹出数组长度

    jmp return                 ; 返回

merge:
    mov eax, ecx               ; 复制数组长度
    mov ebx, esi               ; 设置左侧数组指针
    mov ecx, edi               ; 设置右侧数组指针
    mov edi, sorted_array      ; 设置目标数组指针

merge_loop:
    cmp ebx, esi               ; 检查左侧数组是否已经遍历完
    je copy_remaining_elements ; 如果是，跳转到复制剩余元素的步骤

    cmp ecx, edi               ; 检查右侧数组是否已经遍历完
    je copy_remaining_elements ; 如果是，跳转到复制剩余元素的步骤

    mov al, [ebx]              ; 从左侧数组加载元素到 al 寄存器
    mov dl, [ecx]              ; 从右侧数组加载元素到 dl 寄存器

    cmp al, dl                 ; 比较左侧数组元素和右侧数组元素
    jle copy_left_element      ; 如果左侧数组元素小于等于右侧数组元素，跳转到复制左侧元素的步骤

copy_right_element:
    mov [edi], dl              ; 复制右侧数组元素到目标数组
    inc ecx                    ; 右侧数组指针加1
    inc edi                    ; 目标数组指针加1
    jmp merge_loop             ; 跳转到循环的开头

copy_left_element:
    mov [edi], al              ; 复制左侧数组元素到目标数组
    inc ebx                    ; 左侧数组指针加1
    inc edi                    ; 目标数组指针加1
    jmp merge_loop             ; 跳转到循环的开头

copy_remaining_elements:
    cmp ebx, esi               ; 检查左侧数组是否已经遍历完
    je copy_right_elements     ; 如果是，跳转到复制右侧数组剩余元素的步骤

copy_left_elements:
    mov al, [ebx]              ; 从左侧数组加载元素到 al 寄存器
    mov [edi], al              ; 复制左侧数组元素到目标数组
    inc ebx                    ; 左侧数组指针加1
    inc edi                    ; 目标数组指针加1
    jmp copy_remaining_elements ; 跳转到复制剩余元素的步骤

copy_right_elements:
    mov dl, [ecx]              ; 从右侧数组加载元素到 dl 寄存器
    mov [edi], dl              ; 复制右侧数组元素到目标数组
    inc ecx                    ; 右侧数组指针加1
    inc edi                    ; 目标数组指针加1
    jmp copy_remaining_elements ; 跳转到复制剩余元素的步骤

return:
    ; 在这里处理排序后的数组
    ; 返回到调用者
    ; 可以在这里添加代码来打印排序后的数组
    mov eax, 1
    xor ebx, ebx
    int 0x80

```



### Quick Sort



#### Rust

```rust
use std::fmt::Debug;
/// quick sort
// Move frist element to the correct place
// Everything lower shoud be before it
// Everthing highter should be after it
// return it's location
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        // move our piovt forward 1,and put this element before it
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", p);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        println!("{:?}", v);
        assert_eq!(vec![1, 3, 4, 6, 8, 11, 13], v);
    }
}

```



#### Java

```java
package com.jason.datastructor.sort;

/**
 * @Description:
 * @author: 贾森
 * @date: 2024年04月28日 上午9:12
 */
public class QuickSort {
    public static void main(String[] args) {
        int[] nums = new int[] { 4, 6, 1, 8, 11, 13, 3 };
        quickSort(nums,0, nums.length-1);
        for (int x : nums) {
            System.out.println(x);
        }

    }
    //快排实现方法
    public static void quickSort(int[] array,int low,int high) {
        int i,j,pivot;
        //结束条件
        if(low >= high) {
            return;
        }
        i = low;
        j = high;
        //选择的节点，这里选择的数组的第一数作为节点
        pivot = array[low];
        while(i<j) {
            //从右往左找比节点小的数，循环结束要么找到了，要么i=j
            while(array[j] >= pivot && i<j) {
                j--;
            }
            //从左往右找比节点大的数，循环结束要么找到了，要么i=j
            while(array[i] <= pivot && i<j) {
                i++;
            }
            //如果i!=j说明都找到了，就交换这两个数
            if(i<j) {
                int temp = array[i];
                array[i] = array[j];
                array[j] = temp;
            }
        }
        //i==j一轮循环结束，交换节点的数和相遇点的数
        array[low] = array[i];
        array[i] = pivot;
        System.out.println("--------------------array");
        for (int x : array) {
            System.out.print(x+",");
        }
        System.out.println("--------------------array");
        System.out.println("low:"+low+",high:"+high+",i:"+i+",j:"+j);
        //数组“分两半”,再重复上面的操作
        quickSort(array,low,i-1);
        quickSort(array,i+1,high);
    }
}

```

#### Golang

```go
package main

import "fmt"

func main() {
	s := []int{4, 6, 1, 8, 11, 13, 3}
	quickSort(s)
	fmt.Println(s)
}

func quickSort(s []int) {
	len := len(s)
	if len < 2 {
		return
	}
	p, trip := 0, len-1
	pivot := s[p] //s[head]就是我们的标尺
	for p < trip {
		if s[p+1] > pivot { //标尺元素遇到大于它的，就把这个元素丢到最右边trip
			s[p+1], s[trip] = s[trip], s[p+1]
			trip--
		} else if s[p+1] < s[p] { //标尺元素遇到小于它的，就换位置，标尺右移动一位。
			s[p], s[p+1] = s[p+1], s[p]
			p++
		} else { //相等不用交换
			p++
		}
	}
	//进过上面的处理，保证了标尺左边的元素都小于等于标尺元素（s[p]），右边的元素大于等于标尺元素。
	quickSort(s[:p])
	quickSort(s[p+1:])

}

```



#### Javascript

```javascript
function quickSort(arr) {
  if (arr.length <= 1) {
    return arr;
  }

  const pivot = arr[0];
  const left = [];
  const right = [];

  for (let i = 1; i < arr.length; i++) {
    if (arr[i] < pivot) {
      left.push(arr[i]);
    } else {
      right.push(arr[i]);
    }
  }

  return quickSort(left).concat(pivot, quickSort(right));
}
const arr = [4, 6, 1, 8, 11, 13, 3];
console.log("排序前的数组:", arr);
const sortedArr = quickSort(arr);
console.log("排序后的数组:", sortedArr);
```



#### TypeScript

```typescript
function quickSort(arr: number[]): number[] {
  if (arr.length <= 1) {
    return arr;
  }

  const pivot = arr[0];
  const left: number[] = [];
  const right: number[] = [];

  for (let i = 1; i < arr.length; i++) {
    if (arr[i] < pivot) {
      left.push(arr[i]);
    } else {
      right.push(arr[i]);
    }
  }

  return [...quickSort(left), pivot, ...quickSort(right)];
}

const arrary: number[] = [64, 34, 25, 12, 22, 11, 90];
console.log("排序前的数组", arrary);
const sortedArray: number[] = quickSort(arrary);
console.log("排序后的数组", sortedArray);
```



#### WebAssembly

首先，让我们编写一个简单的C程序，实现快速排序算法。将这个文件命名为`quick_sort.c`：

```c

#include <stdio.h>

void swap(int* a, int* b) {
    int t = *a;
    *a = *b;
    *b = t;
}

int partition(int arr[], int low, int high) {
    int pivot = arr[high];
    int i = low - 1;
    
    for (int j = low; j <= high - 1; j++) {
        if (arr[j] < pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i + 1], &arr[high]);
    return (i + 1);
}

void quickSort(int arr[], int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);

        quickSort(arr, low, pi - 1);
        quickSort(arr, pi + 1, high);
    }
}

int main() {
    int arr[] = {3, 0, 2, 5, -1, 4, 1};
    int n = sizeof(arr) / sizeof(arr[0]);

    quickSort(arr, 0, n - 1);

    printf("Sorted array: ");
    for (int i = 0; i < n; i++)
        printf("%d ", arr[i]);
    printf("\n");

    return 0;
}
```

接下来，我们将使用Emscripten将这个C程序编译成WebAssembly模块。假设你已经安装了Emscripten并配置好了环境变量。

在命令行中运行以下命令：

```bash
emcc quick_sort.c -o quick_sort.js -s WASM=1 -s EXPORTED_FUNCTIONS="['_quickSort']"
```

这将生成两个文件：`quick_sort.js` 和 `quick_sort.wasm`。其中，`quick_sort.js` 包含了用于加载和运行WebAssembly模块的JavaScript代码。

接着，你可以将这些文件嵌入到你的网页中，并在JavaScript代码中调用快速排序算法。例如：

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebAssembly Quick Sort</title>
</head>
<body>
    <script src="quick_sort.js"></script>
    <script>
        // 导入 WebAssembly 模块
        const Module = require('./quick_sort');

        // 调用快速排序函数
        const arr = [3, 0, 2, 5, -1, 4, 1];
        const n = arr.length;
        const ptr = Module._malloc(n * 4); // 申请内存空间
        Module.HEAP32.set(arr, ptr >> 2); // 将数组复制到内存中
        Module._quickSort(ptr, 0, n - 1); // 调用快速排序函数
        const sortedArr = Module.HEAP32.subarray(ptr >> 2, ptr >> 2 + n); // 从内存中读取排序后的数组
        console.log("Sorted array:", sortedArr);
        Module._free(ptr); // 释放内存
    </script>
</body>
</html>
```

这样，你就可以在网页中使用WebAssembly模块实现的快速排序算法了。

### Insertion Sort

#### Rust

```rust
/// Sorts a mutable slice using in-place insertion sort algorithm.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        let cur = arr[i];
        while j > 0 && cur < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = cur;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::have_same_elements;
    use crate::sort::is_sorted;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        let cloned = arr;
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        let cloned = arr;
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        let cloned = arr;
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        let cloned = arr;
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        let cloned = arr.clone();
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        let cloned = arr.clone();
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}

```



#### Java

```java
package com.jason.datastructor.sort;

import java.lang.reflect.Array;

/**
 * @Description:
 * @author: 贾森
 * @date: 2024年04月29日 下午4:11
 */
public class InsertSort {
    public static void main(String[] args) {
        int[] intArray = {64, 34, 25, 12, 22, 11, 90};
        insertSort(intArray);
        printArray(intArray);

    }

    public static void insertSort(int[] arr){
       for (int i = 1;i<arr.length;i++){
           int j = i;
           while(j>0 && arr[j] < arr[j-1]){
               int temp = arr[j];
               arr[j] = arr[j-1];
               arr[j-1] = temp;
               j--;
           }
       }
    }
    // 打印数组
    public static <T> void printArray(int[] array) {
        for (int element : array) {
            System.out.print(element + " ");
        }
        System.out.println();
    }
}

```



#### Golang

```go
package main
import "fmt"

func main() {
	arr := []int{4, 5, 3, 2, 1, 8, 9, 10, 11, 7, 6}
	fmt.Println("排序前的数组：", arr)
	insert_sort(arr)
	fmt.Println("排序后的数组：", arr)
}
func insert_sort(arr []int) {
	n := len(arr)
	for i := 1; i < n; i++ {
		for j := i; j > 0; j-- {
			if arr[j] < arr[j-1] {
				arr[j], arr[j-1] = arr[j-1], arr[j]
			}
		}
	}
}
```



#### JavaScript

```javascript
function insertSort(arr) {
  const n = arr.length;
  for (let i = 1; i < n; i++) {
    for (let j = i; j > 0; j--) {
      if (arr[j] < arr[j - 1]) {
        const temp = arr[j - 1];
        arr[j - 1] = arr[j];
        arr[j] = temp;
      }
    }
  }
  //console.log("排序后的数组:", arr);
}
const arr = [4, 6, 1, 8, 11, 13, 3, 7, 9, 8, 10];
console.log("排序前的数组:", arr);
insertSort(arr);
console.log("排序后的数组:", arr);

```



#### TypeScript

```typescript
function insertSort(arr: Number[]) {
  const n = arr.length;
  for (let i = 1; i < n; i++) {
    for (let j = i; j > 0; j--) {
      if (arr[j] < arr[j - 1]) {
        const temp = arr[j - 1];
        arr[j - 1] = arr[j];
        arr[j] = temp;
      }
    }
  }
}
const arr = [4, 6, 1, 8, 11, 13, 3, 7, 9, 8, 10];
console.log("排序前的数组:", arr);
insertSort(arr);
console.log("排序后的数组:", arr);
```



#### WebAssembly

首先，让我们编写一个简单的C程序，实现插入排序算法。将这个文件命名为 `insertion_sort.c`：

```c

#include <stdio.h>

void insertionSort(int arr[], int n) {
    int i, key, j;
    for (i = 1; i < n; i++) {
        key = arr[i];
        j = i - 1;

        /* Move elements of arr[0..i-1], that are greater than key,
        to one position ahead of their current position */
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key;
    }
}

int main() {
    int arr[] = {12, 11, 13, 5, 6};
    int n = sizeof(arr) / sizeof(arr[0]);

    insertionSort(arr, n);

    printf("Sorted array: ");
    for (int i = 0; i < n; i++)
        printf("%d ", arr[i]);
    printf("\n");

    return 0;
}
```

接下来，我们将使用Emscripten将这个C程序编译成WebAssembly模块。假设你已经安装了Emscripten并配置好了环境变量。

在命令行中运行以下命令：

```bash
emcc insertion_sort.c -o insertion_sort.js -s WASM=1 -s EXPORTED_FUNCTIONS="['_insertionSort']"
```

这将生成两个文件：`insertion_sort.js` 和 `insertion_sort.wasm`。其中，`insertion_sort.js` 包含了用于加载和运行WebAssembly模块的JavaScript代码。

接着，你可以将这些文件嵌入到你的网页中，并在JavaScript代码中调用插入排序算法。例如：

```javascript
html
Copy code
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebAssembly Insertion Sort</title>
</head>
<body>
    <script src="insertion_sort.js"></script>
    <script>
        // 导入 WebAssembly 模块
        const Module = require('./insertion_sort');

        // 调用插入排序函数
        const arr = [12, 11, 13, 5, 6];
        const n = arr.length;
        const ptr = Module._malloc(n * 4); // 申请内存空间
        Module.HEAP32.set(arr, ptr >> 2); // 将数组复制到内存中
        Module._insertionSort(ptr, n); // 调用插入排序函数
        const sortedArr = Module.HEAP32.subarray(ptr >> 2, ptr >> 2 + n); // 从内存中读取排序后的数组
        console.log("Sorted array:", sortedArr);
        Module._free(ptr); // 释放内存
    </script>
</body>
</html>
```

这样，你就可以在网页中使用WebAssembly模块实现的插入排序算法了。



#### asm

```assembly
section .data
    array dd 12, 11, 13, 5, 6
    array_len equ ($ - array) / 4 ; 数组长度

section .text
    global _start

_start:
    mov rsi, array ; rsi 指向数组的首地址
    mov ecx, array_len ; ecx 存储数组长度
    call insertion_sort ; 调用插入排序函数
    jmp end ; 跳转到结束

insertion_sort:
    mov eax, 1 ; eax 为循环变量 i，从第二个元素开始
outer_loop:
    cmp eax, ecx ; 检查是否遍历完整个数组
    jge end_outer_loop ; 如果已经遍历完，则跳出外层循环

    mov ebx, eax ; ebx 存储当前元素的索引 i
    mov edx, [rsi + 4 * ebx] ; edx 存储当前元素的值

    mov edi, eax ; edi 为循环变量 j，从当前元素的前一个元素开始
inner_loop:
    cmp edi, 0 ; 检查是否到达数组开头
    jl end_inner_loop ; 如果到达数组开头，则跳出内层循环

    mov esi, [rsi + 4 * edi] ; esi 存储当前比较的元素的值
    cmp esi, edx ; 比较当前元素和当前比较的元素的值
    jle end_inner_loop ; 如果当前元素大于等于当前比较的元素，则跳出内层循环

    mov [rsi + 4 * (edi + 1)], esi ; 将当前比较的元素往后移动一位
    sub edi, 1 ; 准备比较前一个元素
    jmp inner_loop ; 继续内层循环

end_inner_loop:
    mov [rsi + 4 * (edi + 1)], edx ; 将当前元素插入到合适的位置
    add eax, 1 ; 准备处理下一个元素
    jmp outer_loop ; 继续外层循环

end_outer_loop:
    ret

end:
    ; 在此处添加程序结束的代码，比如输出排序后的数组等

```

### Selection  Sort

#### Rust

```rust
fn select_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        select_sort(&mut v);
        println!("{:?}", v);
        assert_eq!(vec![1, 3, 4, 6, 8, 11, 13], v);
    }
}

```



#### Java

```java
package com.jason.datastructor.sort;

/**
 * @Description:
 * @author: 贾森
 * @date: 2024年04月30日 上午11:15
 */
public class SelectSort {
    public static void main(String[] args) {
        int[] intArray = {64, 34, 25, 12, 22, 11, 90};
        selectSort(intArray);
        printArray(intArray);
    }
    // 打印数组
    public static <T> void printArray(int[] array) {
        for (int element : array) {
            System.out.print(element + " ");
        }
        System.out.println();
    }
    public static void  selectSort(int[] arr){
        for (int i = 0;i<arr.length;i++){
            int smallest = i;
            for (int j=i;j<arr.length;j++){
                if (arr[j] < arr[smallest]){
                    smallest = j;
                }
            }
            int temp = arr[smallest];
            arr[smallest] = arr[i];
            arr[i] = temp;
        }
    }
}
```



#### Golang

```go
import "fmt"

func main() {
	arr := []int{4, 5, 3, 2, 1, 8, 9, 10, 11, 7, 6}
	fmt.Println("排序前的数组：", arr)
	selectSort(arr)
	fmt.Println("排序后的数组：", arr)
}
func selectSort(arr []int) {
	len := len(arr)
	for i := 0; i < len; i++ {
		smallest := i
		for j := i; j < len; j++ {
			if arr[j] > arr[smallest] {
				smallest = j
			}
		}
		arr[smallest], arr[i] = arr[i], arr[smallest]
	}
}

```



#### JavaScript

```javascript
function selectSort(arr) {
  const len = arr.length;
  for (let i = 0; i < len; i++) {
    let smallest = i;
    for (let j = i; j < len; j++) {
      if (arr[j] < arr[smallest]) {
        smallest = j;
      }
    }
    let temp = arr[smallest];
    arr[smallest] = arr[i];
    arr[i] = temp;
  }
}
const arr = [4, 6, 1, 8, 11, 13, 3];
console.log("排序前的数组:", arr);
selectSort(arr);
console.log("排序后的数组:", arr);
```



#### TypeScript

```typescript
function selectSort(arr: Number[]) {
  const len = arr.length;
  for (let i = 0; i < len; i++) {
    let smallest = i;
    for (let j = i; j < len; j++) {
      if (arr[j] < arr[smallest]) {
        smallest = j;
      }
    }
    let temp = arr[smallest];
    arr[smallest] = arr[i];
    arr[i] = temp;
  }
}
const arr1 = [4, 6, 1, 8, 11, 13, 3];
console.log("排序前的数组:", arr1);
selectSort(arr1);
console.log("排序后的数组:", arr1);

```



#### WebAssembly

首先，让我们编写一个简单的C程序，实现选择排序算法。假设我们将这个文件命名为 `selection_sort.c`：

```c
#include <stdio.h>

void selectionSort(int arr[], int n) {
    int i, j, min_idx;

    // One by one move boundary of unsorted subarray
    for (i = 0; i < n-1; i++) {
        // Find the minimum element in unsorted array
        min_idx = i;
        for (j = i+1; j < n; j++)
            if (arr[j] < arr[min_idx])
                min_idx = j;

        // Swap the found minimum element with the first element
        int temp = arr[min_idx];
        arr[min_idx] = arr[i];
        arr[i] = temp;
    }
}

int main() {
    int arr[] = {12, 11, 13, 5, 6};
    int n = sizeof(arr) / sizeof(arr[0]);

    selectionSort(arr, n);

    printf("Sorted array: ");
    for (int i = 0; i < n; i++)
        printf("%d ", arr[i]);
    printf("\n");

    return 0;
}
```

接下来，我们将使用Emscripten将这个C程序编译成WebAssembly模块。假设你已经安装了Emscripten并配置好了环境变量。

在命令行中运行以下命令：

```bash
emcc selection_sort.c -o selection_sort.js -s WASM=1 -s EXPORTED_FUNCTIONS="['_selectionSort']"
```

这将生成两个文件：`selection_sort.js` 和 `selection_sort.wasm`。其中，`selection_sort.js` 包含了用于加载和运行WebAssembly模块的JavaScript代码。

接着，你可以将这些文件嵌入到你的网页中，并在JavaScript代码中调用选择排序算法。例如：

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebAssembly Selection Sort</title>
</head>
<body>
    <script src="selection_sort.js"></script>
    <script>
        // 导入 WebAssembly 模块
        const Module = require('./selection_sort');

        // 调用选择排序函数
        const arr = [12, 11, 13, 5, 6];
        const n = arr.length;
        const ptr = Module._malloc(n * 4); // 申请内存空间
        Module.HEAP32.set(arr, ptr >> 2); // 将数组复制到内存中
        Module._selectionSort(ptr, n); // 调用选择排序函数
        const sortedArr = Module.HEAP32.subarray(ptr >> 2, ptr >> 2 + n); // 从内存中读取排序后的数组
        console.log("Sorted array:", sortedArr);
        Module._free(ptr); // 释放内存
    </script>
</body>
</html>
```

这样，你就可以在网页中使用WebAssembly模块实现的选择排序算法了。



#### Assembly

```assembly
section .data
    array dd 12, 11, 13, 5, 6
    array_len equ ($ - array) / 4 ; 数组长度

section .text
    global _start

_start:
    mov esi, array ; esi 指向数组的首地址
    mov ecx, array_len ; ecx 存储数组长度
    call selection_sort ; 调用选择排序函数
    jmp end ; 跳转到结束

selection_sort:
    mov eax, 0 ; eax 为循环变量 i，从第一个元素开始
outer_loop:
    cmp eax, ecx ; 检查是否遍历完整个数组
    jge end_outer_loop ; 如果已经遍历完，则跳出外层循环

    mov ebx, eax ; ebx 存储当前最小元素的索引
    mov edx, [esi + 4 * ebx] ; edx 存储当前最小元素的值

    mov edi, eax ; edi 为循环变量 j，从当前元素的下一个元素开始
inner_loop:
    cmp edi, ecx ; 检查是否到达数组末尾
    jge end_inner_loop ; 如果到达数组末尾，则跳出内层循环

    mov esi_temp, esi ; 保存数组指针
    add esi_temp, edi ; 移动数组指针到当前元素位置
    mov esi_temp, [esi_temp] ; 读取当前比较的元素的值
    cmp esi_temp, edx ; 比较当前元素和当前最小元素的值
    jl update_min ; 如果当前元素小于当前最小元素，则更新最小元素和索引

update_min:
    mov ebx, edi ; 更新最小元素的索引
    mov edx, esi_temp ; 更新最小元素的值

end_inner_loop:
    ; 交换当前元素和最小元素
    mov esi_temp, esi ; 保存数组指针
    add esi_temp, eax ; 移动数组指针到当前元素位置
    mov esi_temp, [esi_temp] ; 读取当前元素的值
    mov [esi_temp], edx ; 将最小元素放到当前位置
    mov esi_temp, esi ; 保存数组指针
    add esi_temp, ebx ; 移动数组指针到最小元素位置
    mov esi_temp, [esi_temp] ; 读取最小元素的值
    mov [esi_temp], eax ; 将当前元素放到最小元素的位置

    add eax, 1 ; 准备处理下一个元素
    jmp outer_loop ; 继续外层循环

end_outer_loop:
    ret

end:
    ; 在此处添加程序结束的代码，比如输出排序后的数组等

```



### Redix Sort

（1）概念：基数排序（Radix sort）是一种非比较型整数排序算法，其原理是将整数按位数切割成不同的数字，然后按每个位数分别比较。
基数排序是非比较型整数排序算法，其原理是将整数按位分割进行排序。基数排序适用于大范围数据排序，打破了计数排序的限制。由于整数也可以表达字符串（比如名字或日期）和特定格式的浮点数，所以基数排序也不是只能使用于整数。
（2）2种排序方式：
最低位优先法(LSD)：从最低位向最高位依次按位进行排序。
最高位优先法(MSD)：从最高位向最低位依次按位进行排序。
（3）按位分割小技巧
arr[i] / digit % 10，其中digit为10^n。



#### Rust

```rust
/// Sorts the elements of `arr` in-place using radix sort.
///
/// Time complexity is `O((n + b) * logb(k))`, where `n` is the number of elements,
/// `b` is the base (the radix), and `k` is the largest element.
/// When `n` and `b` are roughly the same maginitude, this algorithm runs in linear time.
///
/// Space complexity is `O(n + b)`.
pub fn redix_sort(arr: &mut [u64]) {
    let max: usize = match arr.iter().max() {
        Some(&x) => x as usize,
        None => return,
    };
    // Make radix a power of 2 close to arr.len() for optimal runtime
    let radix = arr.len().next_power_of_two();
    println!("{:?}", radix);
    // Counting sort by each digit from least to most significant
    let mut place = 1;
    while place < max {
        let degit_of = |x| x as usize / place % radix;
        // Count digit occurrenses
        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[degit_of(x)] += 1;
        }
        // Compute last index of each digit
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }
        // Write elements to their new indices
        for &x in arr.to_owned().iter().rev() {
            counter[degit_of(x)] -= 1;
            arr[counter[degit_of(x)]] = x;
        }
        place *= radix;
    }
}
#[cfg(test)]
mod tests {
    use super::redix_sort;

    #[test]
    fn descending() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        redix_sort(&mut v);
        println!("{:?}", v);
        assert_eq!(vec![1, 3, 4, 6, 8, 11, 13], v);
    }
}

```



#### Java

```java
package com.jason.datastructor.sort;

/**
 * @Description:
 * @author: 贾森
 * @date: 2024年05月06日 下午5:28
 */
public class RedixSort {
    public static void main(String[] args) {
        int[]data = {73, 22, 93, 43, 55, 14, 28, 65, 39, 81, 33, 100};
        RedixSort.sort(data, 3);
        for(int i = 0; i < data.length; i++) {
            System.out.print(data[i] + "  ");
        }
    }

    private static void sort(int[] number, int d) {
        int k = 0;
        int n = 1;
        int m = 1; //控制键值排序依据在哪一位

        int[][] temp = new int[10][number.length]; // 数组的第一维表示可能的余数0-9
        int[] order = new int[10]; // 数组order[i]用来表示该位是i的数的个数
        while(m <= d){
            for (int i = 0;i < number.length;i++){
                int lsd = (number[i]/n)%10;
                temp[lsd][order[lsd]] = number[i];
                order[lsd]++;
            }
            for (int i = 0;i<10;i++){
                if (order[i]!=0){
                    for (int j = 0; j < order[i] ;j++){
                        number[k] = temp[i][j];
                        k++;
                    }
                    order[i] = 0;
                }
            }
            n*=10;
            k = 0;
            m++;
        }
    }
}
```



#### Golang

```go
package main

import "fmt"

func main() {
	arr := []int{4, 6, 1, 8, 11, 13, 3, 15, 19, 16, 5}
	radixSort(arr)
	fmt.Println("排序后的数组：", arr)
}

func radixSort(arr []int) {
	max := arr[0]
	for _, value := range arr {
		if value > max {
			max = value
		}
	}
	radix := 10
	k := 0
	for place := 1; place < max; place *= radix {
		var temp [10][]int
		var order [10]int
		for i := 0; i < len(arr); i++ {
			digit := (arr[i] / place) % radix
			temp[digit] = append(temp[digit], arr[i])
			order[digit] += 1
		}
		for j := 0; j < radix; j++ {
			if order[j] != 0 {
				for m := 0; m < len(temp[j]); m++ {
					arr[k] = temp[j][m]
					k++
				}
			}
		}
		k = 0
	}
}
```



#### JavaScript

```javascript
function redixSort(arr) {
  let max = arr[0];
  for (var i = 0; i < arr.length; i++) {
    if (arr[i] > max) {
      max = arr[i];
    }
  }
  const redix = 10;
  let k = 0;
  for (var place = 1; place < max; place *= redix) {
    let temp = [[]];
    let order = [];
    for (var j = 0; j < arr.length; j++) {
      var degit = Math.floor(arr[j] / place) % redix;
      console.log(degit);
      if (!temp[degit]) {
        temp[degit] = [];
      }
      temp[degit].push(arr[j]);
      if (!order[degit]) {
        order[degit] = 0;
      }
      order[degit] += 1;
    }
    console.log(order);
    console.log(temp);
    for (let m = 0; m < redix; m++) {
      if (order[m]) {
        for (let n = 0; n < temp[m].length; n++) {
          arr[k] = temp[m][n];
          k++;
        }
      }
    }
    k = 0;
  }
}

const arr = [4, 6, 1, 8, 11, 13, 3];
console.log("排序前的数组:", arr);
redixSort(arr);
console.log("排序后的数组:", arr);
```



#### TypeScript

```typescript
function redixSort(arr: number[]) {
  let max = arr[0];
  for (var i = 0; i < arr.length; i++) {
    if (arr[i] > max) {
      max = arr[i];
    }
  }
  const redix = 10;
  let k = 0;
  for (var place = 1; place < max; place *= redix) {
    let temp: number[][] = [[]];
    let order: number[] = [];
    for (var j = 0; j < arr.length; j++) {
      var degit = Math.floor(arr[j] / place) % redix;
      if (!temp[degit]) {
        temp[degit] = [];
      }
      temp[degit].push(arr[j]);
      if (!order[degit]) {
        order[degit] = 0;
      }
      order[degit] += 1;
    }

    for (let m = 0; m < redix; m++) {
      if (order[m]) {
        for (let n = 0; n < temp[m].length; n++) {
          arr[k] = temp[m][n];
          k++;
        }
      }
    }
    k = 0;
  }
}

const arr = [4, 6, 1, 8, 11, 13, 3];
console.log("排序前的数组:", arr);
redixSort(arr);
console.log("排序后的数组:", arr);
```



#### WebAssembly

```c
#include<math.h>

//基数排序
void bucketSort3(int *p, intn)
{
    //获取数组中的最大数
    intmaxNum = findMaxNum(p, n);
    //获取最大数的位数，次数也是再分配的次数。
    intloopTimes = getLoopTimes(maxNum);
    inti;
    //对每一位进行桶分配
    for(i = 1; i <= loopTimes; i++)
    {
        sort2(p, n, i);
    }
}
//获取数字的位数
int getLoopTimes(intnum)
{
    intcount = 1;
    inttemp = num / 10;
    while(temp != 0)
    {
        count++;
        temp = temp / 10;
    }
    returncount;
}
//查询数组中的最大数
int findMaxNum(int *p, intn)
{
    inti;
    intmax = 0;
    for(i = 0; i < n; i++)
    {
        if(*(p + i) > max)
        {
            max = *(p + i);
        }
    }
    returnmax;
}
//将数字分配到各自的桶中，然后按照桶的顺序输出排序结果
void sort2(int *p, intn, intloop)
{
    //建立一组桶此处的20是预设的根据实际数情况修改
    intbuckets[10][20] = {};
    //求桶的index的除数
    //如798个位桶index=(798/1)%10=8
    //十位桶index=(798/10)%10=9
    //百位桶index=(798/100)%10=7
    //tempNum为上式中的1、10、100
    inttempNum = (int)pow(10, loop - 1);
    inti, j;
    for(i = 0; i < n; i++)
    {
        introw_index = (*(p + i) / tempNum) % 10;
        for(j = 0; j < 20; j++)
        {
            if(buckets[row_index][j] == NULL)
            {
                buckets[row_index][j] = *(p + i);
                break;
            }
        }
    }
    //将桶中的数，倒回到原有数组中
    int k = 0;
    for(i = 0; i < 10; i++)
    {
        for(j = 0; j < 20; j++)
        {
            if(buckets[i][j] != NULL)
            {
                *(p + k) = buckets[i][j];
                buckets[i][j] = NULL;
                k++;
            }
        }
    }
}
```

接下来，你需要使用 Emscripten 或其他工具将该 C 代码编译成 WebAssembly 模块。以下是一个使用 Emscripten 编译的示例命令：

```
emcc radix_sort.c -o radix_sort.wasm -s EXPORTED_FUNCTIONS="['_radixSort']"
```

然后，你可以在 JavaScript 中加载并调用该 WebAssembly 模块，实现基数排序的功能。

```javascript
// 加载 WebAssembly 模块
fetch('radix_sort.wasm')
  .then(response => response.arrayBuffer())
  .then(buffer => WebAssembly.instantiate(buffer))
  .then(results => {
    // 获取 WebAssembly 模块实例
    const { instance } = results;

    // 获取基数排序函数
    const radixSort = instance.exports.bucketSort3;

    // 测试基数排序
    const arr = new Int32Array([170, 45, 75, 90, 802, 24, 2, 66]);
    console.log("原始数组：", arr);

    // 调用基数排序函数
    bucketSort3(arr.byteOffset, arr.length);

    // 打印排序后的数组
    console.log("排序后的数组：", arr);
  });
```

这样，你就可以使用 WebAssembly 实现基数排序了。

### Counting  sort

#### Rust

```rust
pub fn counting_sort(arr: &mut [u32], maxval: usize) {
    let mut occurences: Vec<usize> = vec![0; maxval + 1];
    for &data in arr.iter() {
        occurences[data as usize] += 1;
    }
    let mut i = 0;
    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u32;
            i += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::counting_sort;

    #[test]
    fn descending() {
        let mut v = vec![
            47, 71, 96, 94, 10, 74, 44, 49, 85, 36, 53, 56, 47, 25, 27, 46, 45, 47, 76, 48, 85, 82,
            4, 79, 66, 64, 38, 9, 31, 90, 18, 72, 88, 85, 44, 30, 49, 93, 58, 82, 13, 24, 46, 94,
            68, 82, 14, 28, 43, 81, 46, 27, 42, 44, 23, 62, 88, 51, 43, 98, 24, 65, 45, 60, 80, 29,
            15, 33, 24, 28, 28, 22, 13, 67, 41, 13, 66, 66, 75, 16, 80, 68, 10, 20, 25, 28, 2, 85,
            32, 32, 46, 27, 88, 51, 40, 51, 39, 82, 47, 24, 68, 42, 1, 97, 50, 67, 77, 52, 100, 5,
            45, 25, 57, 72, 73, 99, 91, 86, 76, 90, 33, 48, 89, 77, 50, 81, 98, 54, 56, 52, 97, 71,
            25, 80, 2, 10, 34, 57, 90, 73, 51, 54, 46, 7, 29, 76, 36, 18, 72, 76, 73, 69, 57, 22,
            99, 59, 67, 38, 5, 25, 63, 61, 9, 39, 95, 16, 22, 25, 32, 18, 23, 39, 81, 80, 100, 70,
            63, 9, 35, 70, 88, 91, 98, 30, 9, 38, 53, 91, 19, 27, 100, 51, 7, 47, 92, 35, 54, 35,
            70, 52, 3, 68, 51, 41, 18, 55, 27, 41, 98, 50, 97, 59,
        ];
        counting_sort(&mut v, 100);
        println!("{:?}", v);
        assert_eq!(
            vec![
                1, 2, 2, 3, 4, 5, 5, 7, 7, 9, 9, 9, 9, 10, 10, 10, 13, 13, 13, 14, 15, 16, 16, 18,
                18, 18, 18, 19, 20, 22, 22, 22, 23, 23, 24, 24, 24, 24, 25, 25, 25, 25, 25, 25, 27,
                27, 27, 27, 27, 28, 28, 28, 28, 29, 29, 30, 30, 31, 32, 32, 32, 33, 33, 34, 35, 35,
                35, 36, 36, 38, 38, 38, 39, 39, 39, 40, 41, 41, 41, 42, 42, 43, 43, 44, 44, 44, 45,
                45, 45, 46, 46, 46, 46, 46, 47, 47, 47, 47, 47, 48, 48, 49, 49, 50, 50, 50, 51, 51,
                51, 51, 51, 51, 52, 52, 52, 53, 53, 54, 54, 54, 55, 56, 56, 57, 57, 57, 58, 59, 59,
                60, 61, 62, 63, 63, 64, 65, 66, 66, 66, 67, 67, 67, 68, 68, 68, 68, 69, 70, 70, 70,
                71, 71, 72, 72, 72, 73, 73, 73, 74, 75, 76, 76, 76, 76, 77, 77, 79, 80, 80, 80, 80,
                81, 81, 81, 82, 82, 82, 82, 85, 85, 85, 85, 86, 88, 88, 88, 88, 89, 90, 90, 90, 91,
                91, 91, 92, 93, 94, 94, 95, 96, 97, 97, 97, 98, 98, 98, 98, 99, 99, 100, 100, 100
            ],
            v
        );
    }
}

```

