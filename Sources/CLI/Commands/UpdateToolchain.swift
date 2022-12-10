//
//  UpdateToolchain.swift
//  
//
//  Created by Shota Shimazu on 2022/12/11.
//

import ArgumentParser


struct UpdateToolchain: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Update depended toolchains"
    )

    func run() throws {
        do {
            try Shell.run("brew update")
            try Shell.run("brew upgrade")
        }
        catch {
            print("Some command has been finished in fail.")
        }
    }
}
