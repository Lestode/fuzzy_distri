IMAGE_NAME=fuzzy-distri
CONTAINER_NAME=fuzzy-distri
PORTS=-p 8080:8080
VOLUMES=-v $(PWD):/app

# Docker build command
build:
	docker build -t $(IMAGE_NAME) .

# Docker run command
run:
	docker run -it --name $(CONTAINER_NAME) $(PORTS) $(VOLUMES) $(IMAGE_NAME)

# Docker stop and remove container command
clean:
	docker stop $(CONTAINER_NAME)
	docker rm $(CONTAINER_NAME)

# Check container logs
logs:
	docker logs $(CONTAINER_NAME)