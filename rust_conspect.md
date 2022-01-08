## Result<T, E> vs. Option<T>

`Result`  содержит либо результат либо ошибку, а
`Option`  содержит либо результат либо ничего. 
Последний используется когда результата может не быть.
		
		"В то время как Result содержит значение ошибки, иногда у вас либо есть что-то, либо нет, и в этих сценариях Option является более подходящим типом для использования." Fullstack Rust.

## attributes

Attributes are the way of attaching metadata to a variety of things in the language.

	Атрибуты — это способ прикрепления метаданных к множеству элементов языка.

They can be attached to modules as a whole, structs, functions, and several other constructs.
 
 	Их можно присоединять к модулям целиком, структурам, функциям и некоторым другим конструкциям.

They can attach to the thing they are defined within using the syntax `#![...]` with a `!` after the `#`. For example,

	Они могут присоединяться к тому, в чем они определены, используя синтаксис `#![...]` с `!` после `#`. Например,

* `#![allow(unused_variables)]` -- `#![разрешить(неиспользуемые_переменные)]`

* `#[derive(Debug)]` -- allows the struct to be printed using the `{:?}`

		Любая структура может быть производной от Debug, если ее составляющие реализуют трейт Debug, который делают все встроенные типы.

* `#[derive(Serialize)]`

		когда мы получили Serialize, любой экземпляр нашей структуры может быть сериализован serde в выходной формат по нашему выбору (если, конечно, мы подключили крейт `serde`). 

Атрибут `derive`, вероятно, является наиболее распространенным атрибутом, с которым вы столкнетесь. Это позволяет вам реализовывать типажи для типов без необходимости выполнять какую-либо дополнительную работу при условии, что тип соответствует требованиям, предъявляемым к свойствам, которые должны быть получены.

## Константы.

Константы, в отличие от переменных, не могут получать значение, вычисленное во время выполнения. Это основная (если я всё правильно понял) разница с неизменяемыми переменными. Поэтому, константам нельзя присвоить переменную окружения? (ИЛИ МОЖНО? ПРОВЕРИТЬ!!)


## Затенение переменных

Разница с перегрузкрой в других языках в том, что в rust-е создаётся новая переменная, с тем же именем. (А в других языках как? Надо полюбопытствовать.) 
Можно использовать с разными типами данных и выражениями, в отличие от `mut`. 


## empty tuple () // пустой кортеж ()

> ### Fullstack Rust // Making Your First Rust App
>#### https://www.newline.co/books/fullstack-rust/making-your-first-rust-app#mainrs

Leaving off the return type is equivalent to writing -> () after the argument list of the function. All function calls are expressions which must return a value. The empty tuple () is a marker for no value, which is what a function with no return type implicitly returns.

Отсутствие возвращаемого типа эквивалентно написанию -> () после списка аргументов функции. Все вызовы функций являются выражениями, которые должны возвращать значение. Пустой кортеж () является маркером отсутствия значения, которое неявно возвращает функция без возвращаемого типа.

> ### Fullstack Rust // Making A Web App With Actix // Ok
> #### https://www.newline.co/books/fullstack-rust/making-a-web-app-with-actix#ok

The Ok variant has type () which is known as the empty tuple which we have seen before. This is commonly used as a marker where you don't actually care about the value but you need some placeholder type. 

Вариант Ok имеет тип (), который известен как пустой кортеж, который мы видели ранее. Это обычно используется в качестве маркера, когда вам на самом деле не нужно значение, но вам нужен какой-то тип заполнителя.
___

This Result type basically means that you can tell the difference between a success and a failure, and if there is a failure you will get an error that can tell you what happened, but in the success case there isn't anything interesting to add besides the fact that there was a success. This pattern is similar to what we see in C with functions that return 0 on success and non-zero on failure where each non-zero value maps to some specific type of failure.

Этот тип результата в основном означает, что вы можете определить разницу между успехом и неудачей, и если есть ошибка, вы получите сообщение об ошибке, которое может сказать вам, что произошло, но в случае успеха нет ничего интересного, кроме как добавить факт, что был успех. Этот шаблон похож на то, что мы видим в C с функциями, которые возвращают 0 в случае успеха и ненулевое значение в случае ошибки, где каждое ненулевое значение отображается на определенный тип ошибки.
___

## `self`
### return type Self
> ### Self is special
> #### https://www.newline.co/books/fullstack-rust/making-a-web-app-with-actix#self-is-special

The first method that we define is called new which takes a port parameter and returns the special type Self. Inside an impl block Self has special meaning, it refers to the type on which we are defining the implementation. So we could have written the signature of new as:

Первый определяемый нами метод называется `new`, который принимает параметр `port` и возвращает специальный тип `Self`. Внутри блока `impl` `Self` имеет особое значение, оно относится к типу, для которого мы определяем реализацию. Таким образом, мы могли бы написать сигнатуру new как:

```rust
pub fn new(port: u16) -> MessageApp
```

However idiomatic Rust code uses Self for such a return type. 

Однако идиоматический код Rust использует `Self` для такого возвращаемого типа. 
___

### special values `self`
> ### All of the selfs
> #### https://www.newline.co/books/fullstack-rust/making-a-web-app-with-actix#all-of-the-selfs

Due to the semantics around borrowing and mutability, there are four special first parameter values: &self, self, &mut self, and mut self. All of the forms turn a function in a method on an instance of the type. This means that rather than being a function on the type which is called like MessageApp::new, we need to have constructed an instance of the type and then use dot syntax to call the method and set the first parameter.

Из-за семантики заимствования и изменчивости существует четыре специальных значения первого параметра: `&self`, `self`, `&mut self` и `mut self`. Все формы превращают функцию в метод для экземпляра типа. Это означает, что вместо того, чтобы быть функцией типа, который вызывается как `MessageApp::new`, нам нужно создать экземпляр типа, а затем использовать точечный синтаксис для вызова метода и установки первого параметра.

The first form, &self, is the most common form. This means that our method takes an immutable reference to the instance invoking the method. We can read the data inside our type, but we cannot alter it. The calling code also maintains ownership so we are just borrowing the instance.


Первая форма, `&self`, является наиболее распространенной формой. Это означает, что наш метод принимает неизменяемую ссылку на экземпляр, вызывающий метод. Мы можем читать данные внутри нашего типа, но не можем их изменить. Вызывающий код также сохраняет право собственности, поэтому мы просто заимствуем экземпляр.