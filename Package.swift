// swift-tools-version: 5.6
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription
import Foundation


func getRelativePathToHome() -> String {
    let fm = FileManager.default
    var relativePath = ""
    var pwd = fm.currentDirectoryPath
    let home = fm.homeDirectoryForCurrentUser.path
    
    let result = pwd.range(of: home)
    if let theRange = result {
        pwd.removeSubrange(theRange)
    }
    
    let diff = pwd.components(separatedBy: "/").filter{!$0.isEmpty}

    for _ in diff {
        relativePath += "../"
    }

    return relativePath
}


let cxxSettings: [CXXSetting] = [
    .headerSearchPath("."),
    .headerSearchPath("include"),
    .headerSearchPath("\(getRelativePathToHome()).wasmer/include"),
]


let package = Package(
    name: "CoreOverlay",
    platforms: [
        .macCatalyst(.v14),
        .macOS(.v11),
        .iOS(.v14),
        .watchOS(.v7),
        .tvOS(.v14),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "CoreOverlay",
            targets: ["CoreOverlay"]
        ),
        .library(
            name: "OverlayDB",
            targets: ["OverlayDB"]
        ),
        .library(
            name: "CLevelDB",
            targets: ["CLevelDB"]
        ),
        .executable(
            name: "cot",
            targets: ["CLI"]
        ),
    ],
    dependencies: [
        // Basic dependencies
        .package(url: "https://github.com/grpc/grpc-swift.git", from: "1.9.0"),
        .package(url: "https://github.com/apple/swift-crypto.git", "1.0.0" ..< "3.0.0"),
        .package(url: "https://github.com/stephencelis/SQLite.swift.git", .upToNextMajor(from: "0.9.2")),
        .package(url: "https://github.com/shareup/wasm-interpreter-apple.git", from: "0.5.3"),
        .package(url: "https://github.com/apple/swift-argument-parser", from: "1.2.0"),
        .package(url: "https://github.com/weichsel/ZIPFoundation.git", .upToNextMajor(from: "0.9.0")),
        //- .package(url: "https://github.com/swift-libp2p/swift-libp2p.git", .upToNextMajor(from: "0.0.1")),

        // Development only dependencies
        .package(url: "https://github.com/nicklockwood/SwiftFormat", from: "0.49.0"),
        // .package(url: "https://github.com/realm/SwiftLint.git", from: "0.39.2"),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .systemLibrary(
            name: "CLevelDB",
            providers: [
                .brew(["leveldb"]),
            ]
        ),
        .binaryTarget(
            name: "CWasmer",
            url: "https://github.com/shotastage/CWasmer/releases/download/v0.0.1/CWasmer.xcframework.zip",
            checksum: "b91a858ee7ff1ee9ec1fd0b9da126cd1bfde4e09f9ae75a4ad0364fdcbdd27f3"
        ),
        .binaryTarget(
            name: "CoreOverlayEngine",
            url: "https://github.com/shotastage/CoreOverlay/releases/download/v0.0.4/bundle.zip",
            checksum: "bfe73fa008052eafab0857c74e83895484f26aa344b7f96f1717ace6f9290843"
        ),
        .target(
            name: "CommonCrypt"
        ),
        .target(
            name: "OverlayFundation"
        ),
        .target(
            name: "OverlayDB",
            dependencies: [
                .product(name: "SQLite", package: "SQLite.swift"),
            ]
        ),
        .target(
            name: "Runtime",
            dependencies: [
                "CWasmer",
                .product(name: "WasmInterpreter", package: "wasm-interpreter-apple"),
            ]
        ),
        .target(
            name: "Protobuf.Generated",
            dependencies: [
                .product(name: "GRPC", package: "grpc-swift"),
            ]
        ),
        .target(
            name: "CoreOverlay",
            dependencies: [
                "CommonCrypt",
                "OverlayFundation",
                "CoreOverlayEngine",
                "Protobuf.Generated",
                "Runtime",
                .product(name: "GRPC", package: "grpc-swift"),
                .product(name: "Crypto", package: "swift-crypto"),
                .product(name: "WasmInterpreter", package: "wasm-interpreter-apple"),
                //- .product(name: "LibP2P", package: "swift-libp2p"),
            ]
        ),
        .executableTarget(
            name: "CLI",
            dependencies: [
                .product(name: "ArgumentParser", package: "swift-argument-parser"),
                .product(name: "ZIPFoundation", package: "ZIPFoundation"),
            ]
        ),
        .testTarget(
            name: "CoreOverlayTests",
            dependencies: ["CoreOverlay"]
        ),
    ],
    cxxLanguageStandard: .cxx20
)
