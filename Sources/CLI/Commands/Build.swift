//
//  Build.swift
//  
//
//  Created by Shota Shimazu on 2022/12/01.
//

import ArgumentParser


struct Build: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Build CoreOverlay deployable package."
    )

    func run() throws {
        print("Build Tool is now under construction that it has possibility of being unstable.")

        do {
            try Shell.run("swiftenv global wasm-5.7.1")
            try Shell.run("swiftc -target wasm32-unknown-wasi $1 -o ${1%.*}.wasm")
            try Shell.run("wasm2wat ${1%.*}.wasm -o ${1%.*}.wat")
            try Shell.run("echo $(wat2wasm -o >(base64) ${1%.*}.wat) > ${1%.*}.wb64")
            try Shell.run("mv ${1%.*}.wb64 ./")
            try Shell.run("swiftenv global system")
            try Shell.run("rm ${1%.*}.wasm")
            try Shell.run("rm ${1%.*}.wat")
        }
        catch {
            print("Some command has been finished in fail.")
        }
    }
}
