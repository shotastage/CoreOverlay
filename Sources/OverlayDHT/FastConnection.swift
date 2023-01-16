//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2023/01/17.
//

import Foundation
import Network
import Combine



// Thanks to https://gist.github.com/michael94ellis/92828bba252ccabd071279be098e26e6#file-udplistener
// Modified by Shota Shimazu
class FastConnection: ObservableObject {

    var listener: NWListener?
    var conn: NWConnection?
    var queue = DispatchQueue.global(qos: .userInitiated)
    /// New data will be place in this variable to be received by observers
    @Published private(set) public var messageReceived: Data?
    /// When there is an active listening NWConnection this will be `true`
    @Published private(set) public var isReady: Bool = false
    /// Default value `true`, this will become false if the UDPListener ceases listening for any reason
    @Published public var listening: Bool = true
    
    /// A convenience init using Int instead of NWEndpoint.Port
    convenience init(on port: Int) {
        self.init(on: NWEndpoint.Port(integerLiteral: NWEndpoint.Port.IntegerLiteralType(port)))
    }
    /// Use this init or the one that takes an Int to start the listener
    init(on port: NWEndpoint.Port) {
        let params = NWParameters.udp
        params.allowFastOpen = true
        self.listener = try? NWListener(using: params, on: port)
        self.listener?.stateUpdateHandler = { update in
            switch update {
            case .ready:
                self.isReady = true
                print("Listener connected to port \(port)")
            case .failed, .cancelled:
                // Announce we are no longer able to listen
                self.listening = false
                self.isReady = false
                print("Listener disconnected from port \(port)")
            default:
                print("Listener connecting to port \(port)...")
            }
        }
        self.listener?.newConnectionHandler = { connection in
            print("Listener receiving new message")
            self.createConnection(connection: connection)
        }
        self.listener?.start(queue: self.queue)
    }
    
    func createConnection(host: String, port: UInt16) {
        let nwHost = NWEndpoint.Host(host)
        let nwPort = NWEndpoint.Port(integerLiteral: port)
        
        let connection = NWConnection(host: nwHost, port: nwPort, using: .udp)
        self.conn = connection

        self.conn?.stateUpdateHandler = { (newState) in
            switch (newState) {
            case .ready:
                print("Listener ready to receive message - \(connection)")
                self.receive()
            case .cancelled, .failed:
                print("Listener failed to receive message - \(connection)")
                // Cancel the listener, something went wrong
                self.listener?.cancel()
                // Announce we are no longer able to listen
                self.listening = false
            default:
                print("Listener waiting to receive message - \(connection)")
            }
        }
        self.conn?.start(queue: .global())
    }

    func createConnection(connection: NWConnection) {
        self.conn = connection

        self.conn?.stateUpdateHandler = { (newState) in
            switch (newState) {
            case .ready:
                print("Listener ready to receive message - \(connection)")
                self.receive()
            case .cancelled, .failed:
                print("Listener failed to receive message - \(connection)")
                // Cancel the listener, something went wrong
                self.listener?.cancel()
                // Announce we are no longer able to listen
                self.listening = false
            default:
                print("Listener waiting to receive message - \(connection)")
            }
        }
        self.conn?.start(queue: .global())
    }
    
    func receive() {
        self.conn?.receiveMessage { data, context, isComplete, error in
            if let unwrappedError = error {
                print("Error: NWError received in \(#function) - \(unwrappedError)")
                return
            }
            guard isComplete, let data = data else {
                print("Error: Received nil Data with context - \(String(describing: context))")
                return
            }
            self.messageReceived = data
            if self.listening {
                self.receive()
            }
        }
    }
    
    func cancel() {
        self.listening = false
        self.conn?.cancel()
    }
}

