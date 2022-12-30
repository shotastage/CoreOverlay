func main() {
    print("Hello, world!")
}

func add(a: Int, b: Int) -> Int {
    a + b
}

actor SampleActor {
    let property = 1

    func method() -> Int {
        property + 1
    }

    func method2(msg: String) -> String {
        "Hello \(msg)"
    }
}
