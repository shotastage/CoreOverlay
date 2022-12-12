func add(a: Int, b: Int) -> Int {
    return a + b
}

actor SampleActor {
    let property = 1

    func method() -> Int {
        return property + 1
    }

    func method2(msg: String) -> String {
        return "Hello \(msg)"
    }
}
