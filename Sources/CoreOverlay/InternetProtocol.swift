//
//  InternetProtocol.swift
//  
//
//  Created by Shota Shimazu on 2022/06/08.
//

import Darwin

public enum InternetNetworkProtocol {
    case ipv4
    case ipv6
}

public class COInternetNetwork {
    
    func getIp(protocol: InternetNetworkProtocol = .ipv4) -> String {
        return ""
    }
}
