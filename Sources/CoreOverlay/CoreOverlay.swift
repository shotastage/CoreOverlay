//
//  SwiftlyDHT.swift
//
//
//  Created by Shota Shimazu on 2022/06/08.
//

import COLibs


public struct CoreOverlay {
    public init() {
    }

    func start() {
        COLogger.info("Starting CoreOverlay internal server...")
    }
    
    func shutdown() {
        COLogger.info("Shutting down server...")
    }

    func registerArtifacts() {
        
    }
}

#if DEBUG
class Show {
    func show() {
        print("CoreOverlay")
    }
}
#endif
