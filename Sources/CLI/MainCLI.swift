//
//  main.swift
//  
//
//  Created by Shota Shimazu on 2022/12/01.
//

import ArgumentParser


@main
struct MainCLI: ParsableCommand {
    @Flag(help: "Include a counter with each repetition.")
    var includeCounter = false

    mutating func run() throws {
        print("CLI is now under construction...")
    }
}
