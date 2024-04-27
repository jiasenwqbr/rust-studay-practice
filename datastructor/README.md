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





