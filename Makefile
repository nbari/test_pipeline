.PHONY: help encrypt decrypt clean

help: check-key clean
	@echo "Use encrypt or decrypt"

encrypt: check-key
	@openssl enc -pbkdf2 -in env -out env.enc -e -aes256 -k ${KEY}

decrypt: check-key
	@openssl enc -pbkdf2 -in env.enc -out env -d -aes256 -k ${KEY}

clean:
	@rm -fv env

check-key:
ifndef KEY
	$(error KEY is undefined try: "export KEY=secret")
endif
