.PHONY: help encrypt decrypt clean

help: check-key clean
	@echo "Use encrypt or decrypt"

encrypt: check-key
	@openssl enc -pbkdf2 -in env.yml -out env.yml.enc -e -aes256 -k ${KEY}

decrypt: check-key
	@openssl enc -pbkdf2 -in env.yml.enc -out env.yml -d -aes256 -k ${KEY}

clean:
	@rm -fv env.yml

check-key:
ifndef KEY
	$(error KEY is undefined try: "export KEY=secret")
endif
