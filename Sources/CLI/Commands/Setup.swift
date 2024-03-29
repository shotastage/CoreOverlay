//
//  Setup.swift
//
//
//  Created by Shota Shimazu on 2022/12/02.
//

import ArgumentParser

struct Setup: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Setup CoreOverlay develop environment"
    )

    func run() throws {
        SetupProcedure().run()
    }
}

final class SetupProcedure: CommandProcedure {
    var args: [String] = []

    func prepare() {
        print("Setup Tool is now under construction that it has possibility of being unstable.")
    }

    func procedure() {
        do {
            try Shell.run("brew update")
            try Shell.run("brew install swift-protobuf grpc-swift wabt")
            // Temporary disabled installing swiftenv.
            //- try Shell.run("brew install kylef/formulae/swiftenv")
        } catch {
            print("Some command has been finished in fail.")
        }
    }
}
