//
//  main.swift
//
//
//  Created by Shota Shimazu on 2022/12/01.
//

import ArgumentParser

#if os(macOS)

    @main
    struct MainCLI: ParsableCommand {
        static let configuration: CommandConfiguration = {
            CommandConfiguration(
                commandName: "cot",
                abstract: "CoreOverlay Development Kit",
                version: "0.0.2",
                subcommands: [
                    Build.self,
                    Create.self,
                    Setup.self,
                    UpdateToolchain.self,
                ],
                defaultSubcommand: Build.self
            )
        }()
    }

#else

    @main
    enum MainCLI {
        static func main() {
            print("CoreOverlay Commandline Tools is not supported on your platform.")
        }
    }

#endif
