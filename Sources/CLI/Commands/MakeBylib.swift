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
    case iOSARM64 = "aarch64-apple-ios-sim"
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
            try Shell.run("brew update")
            print("ARG1 \(args[0])")
            try Shell.run("wasmer compile \(args[0]) --target \(ArchtectureLiterals.iOSARM64.rawValue) --dylib  -o build.dylib")
        } catch {
            print("Some command has been finished in fail.")
        }
    }
}
