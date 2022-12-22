//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/12/22.
//

import ArgumentParser

struct MakeBylib: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Setup CoreOverlay develop environment"
    )

    func run() throws {
        MakeBylibProcedure().run()
    }
}

class MakeBylibProcedure: CommandProcedure {
    func prepare() {
        print("Bylib Tool is now under construction that it has possibility of being unstable.")
    }

    func procedure() {
        do {
            try Shell.run("brew update")
        } catch {
            print("Some command has been finished in fail.")
        }
    }
}
