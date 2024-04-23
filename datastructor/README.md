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

