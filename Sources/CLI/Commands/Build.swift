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
        print("Builder is now under construction...")
    }
}
