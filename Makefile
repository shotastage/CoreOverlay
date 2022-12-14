SWIFT:=swift

.PHONY:
run:
	${SWIFT} run

.PHONY:
generate-proto:
	protoc --swift_out=Sources/Protobuf.Generated/ --swift_opt=Visibility=Public --proto_path=Protos/ Protos/*.proto

.PHONY:
build: generate-proto
	#./Tools/generate-modulemap.sh Sources/CWasmer
	${SWIFT} build

.PHONY:
clean:
	rm -rf .build/
	rm -rf .swiftpm/xcode/
	rm ./Sources/CWasmer/include/CWasmer_generated.h
	find ./Sources/Protobuf.Generated/ -type f -name "*.swift" -delete

.PHONY:
release:
	${SWIFT} build -c release

.PHONY:
update-deps:
	rm Package.resolved
	xcodebuild -resolvePackageDependencies

SDKROOT=(xcrun --sdk macosx --show-sdk-path)


.PHONY:
format:
	${SWIFT} package plugin --allow-writing-to-package-directory swiftformat .
