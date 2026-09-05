.PHONY: build check fix test vet clean

BINARY := ecs-go

build:
	go build -o $(BINARY) .

check: build
	./$(BINARY) .

fix: build
	./$(BINARY) --fix .

test:
	go test ./...

vet:
	go vet ./...

clean:
	rm -f $(BINARY)
