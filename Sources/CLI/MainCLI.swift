//
//  main.swift
//  
//
//  Created by Shota Shimazu on 2022/12/01.
//

import ArgumentParser


@main
struct MainCLI: ParsableCommand {
    static let configuration: CommandConfiguration = {
        return CommandConfiguration(
            commandName: "cot",
            abstract: "CoreOverlay Development Kit",
            version: "0.0.2",
            subcommands: [
                Build.self,
                Setup.self,
                UpdateToolchain.self,
            ],
            defaultSubcommand: Build.self
        )
    }()
}
