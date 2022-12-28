//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/12/22.
//

import ArgumentParser

struct MakeBylib: ParsableCommand {
    
    @Argument var args: [String] = []

    static let configuration = CommandConfiguration(
        abstract: "Setup CoreOverlay develop environment"
    )

    func run() throws {
        let procedure = MakeBylibProcedure(arguments: args)
        procedure.run()
    }
}


enum ArchtectureLiterals: String {
    case iOSX86 = "x86_64-apple-ios"
    case iOSARM64 = "_aarch64-apple-ios-sim"
    case MacX86 = "__NOT_DEFINED_FOR_X86_"
    case MacARM64 = "__NOT_DEFINED_FOR_ARM64_"
}


final class MakeBylibProcedure: CommandProcedure {

    var args: [String] = []
    
    required init() {}

    init(arguments: [String]) {
        args = arguments
    }

    func prepare() {
        print("Bylib Tool is now under construction that it has possibility of being unstable.")
    }

    func procedure() {
        do {
            try Shell.run("wasmer compile -o build.dylib \(args[0]) --target \(ArchtectureLiterals.iOSARM64.rawValue) --dylib")
        } catch {
            print("Some command has been finished in fail.")
        }
    }
}
