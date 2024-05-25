java_run: lib
	javac Main.java && java -Djava.library.path=jni_events/target/debug/ Main
# If needed to specify the version of java: javac -source 1.8 -target 1.8 Main.java && ...

.PHONY: jni_events

javah:
	javac -h . Main.java

lib:
	cd jni_events && cargo build
