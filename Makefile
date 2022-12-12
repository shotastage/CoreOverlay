SWIFT:=swift

.PHONY:
run:
	${SWIFT} run

.PHONY:
generate-proto:
	protoc --swift_out=Sources/Protobuf.Generated/ --swift_opt=Visibility=Public --proto_path=Protos/ Protos/*.proto

.PHONY:
build: generate-proto
	${SWIFT} build

.PHONY:
clean:
	rm -rf .build/
	rm -rf .swiftpm/xcode/
	find ./Sources/Protobuf.Generated/ -type f -name "*.swift" -delete

.PHONY:
release:
	${SWIFT} build -c release

.PHONY:
update-deps:
	rm Package.resolved
	xcodebuild -resolvePackageDependencies
