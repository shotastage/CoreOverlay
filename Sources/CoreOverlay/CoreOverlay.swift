//
//  SwiftlyDHT.swift
//
//
//  Created by Shota Shimazu on 2022/06/08.
//

import LibP2P


public struct CoreOverlay {
    public init() {
    }

    deinit() {
        lib.shutdown()
    }

    func initializeP2PNetwork() {
        let lib = try Application(.development, peerID: PeerID(.Ed25519))
        lib.security.use(.noise)
        lib.muxers.use(.mplex)
        lib.servers.use(.tcp(host: "127.0.0.1", port: 0))

        try lib.routes()
        lib.start()

    }
}

#if DEBUG
class Show {
    func show() {
        print("CoreOverlay")
    }
}
#endif
