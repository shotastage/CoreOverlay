//
//  SocketConnectionTests.swift
//
//
//  Created by Shota Shimazu on 2022/06/08.
//

@testable import OverlayFundation
import XCTest

final class SocketConnectionTests: XCTestCase {
    func testXORDistance1() throws {
        let first = 0b00_1000_1010
        let second = 0b00_1000_0010

        print("====== TEST CASE 1 ======")
        print("IN:   \(first)")
        print("OUT:  \(second)")
        print("DIFF: \(first ^ second)")
        print("DIS:  \(COMath.hummingDistance(first, second))")

        XCTAssertEqual(COMath.hummingDistance(first, second), 1)
    }

    func testXORDistance2() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.

        let first = 0b001_0100_0101_0101_1101
        let second = 0b001_0111_0001_0101_1101

        print("====== TEST CASE 2 ======")
        print("IN:   \(first)")
        print("OUT:  \(second)")
        print("DIFF: \(first ^ second)")
        print("DIS:  \(COMath.hummingDistance(first, second))")

        XCTAssertEqual(COMath.hummingDistance(first, second), 3)
    }
}
