//
//  CommonCryptoAlgorithmTests.swift
//
//
//  Created by Shota Shimazu on 2022/06/09.
//

import Foundation

@testable import CoreOverlay
import XCTest

final class CommonCryptoAlgorithmTests: XCTestCase {
    func testSHA1() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.

        let text = "HELLO_WORLD"

        print("====== TEST CASE 1 ======")
        print("TEXT: \(text)")
        print("DIS:  \(CommonCryptoAlgorithm.sha1(data: text.data(using: .utf8)!))")

        XCTAssertEqual(CommonCryptoAlgorithm.sha1(data: text.data(using: .utf8)!), "e94c4822769a728d4e106461b823328f5945f3e0")
    }
}
