//
//  SocketConnectionTests.swift
//  
//
//  Created by Shota Shimazu on 2022/06/08.
//

import XCTest
@testable import CoreOverlay

final class SocketConnectionTests: XCTestCase {
    func testXORDistance1() throws {
        let first =  0b0010001010
        let second = 0b0010000010

        print("====== TEST CASE 1 ======")
        print("IN:   \(first)")
        print("OUT:  \(second)")
        print("DIFF: \(first ^ second)")
        print("DIS:  \(DHTMath.hummingDistance(first, second))")

        XCTAssertEqual(DHTMath.hummingDistance(first, second), 1)
    }

    func testXORDistance2() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.

        let first =  0b0010100010101011101
        let second = 0b0010111000101011101


        print("====== TEST CASE 2 ======")
        print("IN:   \(first)")
        print("OUT:  \(second)")
        print("DIFF: \(first ^ second)")
        print("DIS:  \(DHTMath.hummingDistance(first, second))")

        XCTAssertEqual(DHTMath.hummingDistance(first, second), 3)
    }
}
