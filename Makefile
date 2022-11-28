SWIFT:=swift

.PHONY:
build:
	${SWIFT} build

.PHONY:
run:
	${SWIFT} run

.PHONY:
generate-proto:
	protoc --swift_out=Sources/Generated --swift_opt=Visibility=Public --proto_path=Protos/ Protos/*.proto

.PHONY:
clean:
	rm -rf .build/
