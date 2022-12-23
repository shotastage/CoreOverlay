//
//  PackageBuilder.swift
//
//
//  Created by Shota Shimazu on 2022/12/12.
//

import Foundation
import ZIPFoundation

class PackageBuilder: CommandProcedure {
    var args: [String] = []
    
    required init() {}
    
    func prepare() {
        pack()
    }

    func procedure() {
        fatalError("Not implemented")
    }

    private func pack() {
        let fileManager = FileManager()
        let currentWorkingPath = fileManager.currentDirectoryPath
        var sourceURL = URL(fileURLWithPath: currentWorkingPath)
        sourceURL.appendPathComponent("file.wb1")
        var destinationURL = URL(fileURLWithPath: currentWorkingPath)
        destinationURL.appendPathComponent("build-package.opkg")
        do {
            try fileManager.zipItem(at: sourceURL, to: destinationURL)
        } catch {
            print("Package packing has been failed: \(error)")
        }
    }
}
