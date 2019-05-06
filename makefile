.PHONY: start
start: dockerStart

.PHONE: delete
delete: dockerDelete

.PHONY: dockerStart
dockerStart:
	docker-compose up -d

.PHONE: dockerDelete
dockerDelete:
	docker-compose down -v