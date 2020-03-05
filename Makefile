postgres:
	cd docker && docker-compose down && docker-compose up -d

test_request:
	grpcurl -proto ./grpc/models.proto -d '{"name":"Kiryu Coco"}' -plaintext localhost:50051 user.UserService.NewUser
