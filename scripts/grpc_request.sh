grpcurl -proto src/grpc/models.proto -d '{"name":"Kiryu Coco"}' -plaintext localhost:50051 user.UserService.NewUser
