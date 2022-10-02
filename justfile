
set shell := [ "nu.exe", "-c" ]

watch:
	watchexec -e rs,toml just git

git:
	git add -A
	git commit -m "save"
	
push:
	git push
