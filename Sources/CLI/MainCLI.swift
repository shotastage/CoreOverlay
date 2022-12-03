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
            version: "0.0.1",
            subcommands: [
                Build.self,
                Setup.self,
            ],
            defaultSubcommand: Build.self
        )
    }()
}
