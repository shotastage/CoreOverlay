//
//  Create.swift
//
//
//  Created by Shota Shimazu on 2022/12/12.
//

import ArgumentParser
import Foundation

struct Create: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Create a new CoreOverlay application project."
    )

    func run() throws {
        do {
            try Shell.run("brew update")
        } catch {
            print("Some command has been finished in fail.")
        }
    }
}
