# バックエンド
postgres:
	cd docker && docker-compose down && docker-compose up -d

# フロントエンド
vue_serve:
	cp conf/credentials.json frontend/src/conf && cd frontend && npm run serve;

# その他
test_request:
	grpcurl -proto ./grpc/models.proto -d '{"name":"Kiryu Coco"}' -plaintext localhost:50051 user.UserService.NewUser
