//
//  SocketConnection.swift
//  
//
//  Created by Shota Shimazu on 2023/01/12.
//

import Foundation
import Network

open class SocketClient {
    var conn: NWConnection?

    init() {}

    func connecnt(host: String, port: UInt16) {
        let nwHost = NWEndpoint.Host(host)
        let nwPort = NWEndpoint.Port(integerLiteral: port)
        let semaphore = DispatchSemaphore(value: 0)

        conn = NWConnection(host: nwHost, port: nwPort, using: .udp)

        conn?.stateUpdateHandler = { newState in
            switch newState {
            case .ready:
                semaphore.signal()
            case let .waiting(error):
                NSLog("\(#function), \(error)")
            case let .failed(error):
                NSLog("\(#function), \(error)")
            case .setup: break
            case .cancelled: break
            case .preparing: break
            @unknown default:
                fatalError("Illegal state")
            }
        }

        let queue = DispatchQueue(label: "udp_socket_connection")
        conn?.start(queue: queue)

        semaphore.wait()
    }

    func disconnect() {
        conn?.cancel()
    }

    func send(data: Data) {
        let semaphore = DispatchSemaphore(value: 0)

        conn?.send(content: data, completion: .contentProcessed { error in
            if let error {
                NSLog("\(#function), \(error)")
            } else {
                semaphore.signal()
            }
        })

        semaphore.wait()
    }

    func receive(action: @escaping (Data) -> Void) {
        let semaphore = DispatchSemaphore(value: 0)

        conn?.receive(minimumIncompleteLength: 0, maximumLength: 65535, completion: { data, _, _, error in
            if let error {
                NSLog("\(#function), \(error)")
            } else {
                if let data {
                    action(data)
                    semaphore.signal()
                } else {
                    NSLog("nil message received")
                }
            }
        })
        semaphore.wait()
    }
}
