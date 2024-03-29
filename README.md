# poly-rust

## Dynamic Runtime Design Patterns in Rust

The goal of this project is to create examples of common design patterns in rust that can be applied using runtime polymorphism. Specifically, we're targeting developers who are familiar with C++, Java, or other OOP langagues.

So what does "Runtime Polymorphism" really mean to this project? Specifically, where possible, types can be interchanged at runtime safely.
In C++ we'd let these types be defined in different libraries, and so "Host" code wouldn't know about "Client" code, and we wouldn't use templates, but instead abstact classes. In java, we'd write methods in terms of interfaces, and authors can extend and implement those. At a higher level, we want to adhere to the Liskov Substitution Principle.

### Techniques

* In C++ we typcially make use of pure virtual class pointers to implement run-time polymorphism.
* In Java we have interfaces and method overrides.
* In Rust, we leverage Traits for the role of interfaces, and `Box<T: dyn Trait>` takes the place of pointers--or more specifically `std::unique_ptr`. Along with Generics, this lets you abstract enough information away to get Liskov Substituion at runtime.

Note: These techniques can come with a performance penalty over compile time alternatives. Be sure you need runtime type switching, and that Rust enums aren't sufficient for your useage. You can go a long way with enums, traits, and generics without ever needing to switch types at runtime. Some things you might need runtime polymorphism for are: plugin architectures, type selection based on user/external input, stateful applications.

---

### Patterns

Patterns which are already implemented are marked with a [x]. Issues for examples are linked next to unfinished examples.

---

#### Structural

* [x] Adapter
* [ ] Bridge [#2](https://github.com/jhughes-dev/poly-rust/issues/2)
* [x] Composite
* [x] Decorator
* [ ] Facade [#4](https://github.com/jhughes-dev/poly-rust/issues/4) (Not polymorphic)
* [ ] Flyweight [#5](https://github.com/jhughes-dev/poly-rust/issues/5)
* [x] Proxy

---

#### Behavioral

* [x] Chain of Responsibility
* [x] Command [#8](https://github.com/jhughes-dev/poly-rust/issues/8)
* [ ] Iterator [#9](https://github.com/jhughes-dev/poly-rust/issues/9) (Standard Rust Feature)
* [ ] Mediator [#10](https://github.com/jhughes-dev/poly-rust/issues/10) (Not polymorphic)
* [ ] Memento [#11](https://github.com/jhughes-dev/poly-rust/issues/11)
* [x] Observer
* [ ] State [#13](https://github.com/jhughes-dev/poly-rust/issues/13)
* [x] Strategy
* [x] Template Method
* [x] Visitor

---

#### Creational

* [x] Abstract Factory
* [x] Builder
* [x] Factory Method
* [x] Prototype
* [x] Singleton (Not Polymorphic)
* [x] [Type State Builder](src/typestatebuilder.rs) (Monomorphic Builder -> Possible to build polymorphic similar to builder though)

---

#### Special

* [ ] Model View Controller (MVC) [#19](https://github.com/jhughes-dev/poly-rust/issues/19)


### Links

* https://refactoring.guru/design-patterns/rust - Good Design Pattern Examples in Rust, most use compile time polymorphism.
* https://rust-unofficial.github.io/patterns/patterns/index.html - Rust (un)official design patterns.
* https://www.google.com/books/edition/Design_Patterns/6oHuKQe3TjQC?hl=en&gbpv=0&kptab=getbook - "Gang of Four" Design Patterns Book (great for reference)
* https://www.google.com/books/edition/Head_First_Design_Patterns/GGpXN9SMELMC?hl=en&gbpv=0 - Headfirst Design Patterns Book, (great for learning)

### Citations

Vlissides, John, et al. Design Patterns: Elements of Reusable Object-Oriented Software. India, Pearson Education, 1994.

Bates, Bert, et al. Head First Design Patterns. Germany, O'Reilly Media, Incorporated, 2004.
