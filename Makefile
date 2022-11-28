SWIFT:=swift

.PHONY:
run:
	${SWIFT} run

.PHONY:
generate-proto:
	protoc --swift_out=Sources/Protobuf.Generated/ --swift_opt=Visibility=Public --proto_path=Protos/ Protos/*.proto

.PHONY:
build:
	${SWIFT} build

.PHONY:
clean:
	rm -rf .build/
	find ./Sources/Protobuf.Generated/ -type f -name "*.swift" -delete
