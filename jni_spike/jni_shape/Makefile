java_run: lib
	javac JNIShape.java && java -Djava.library.path=shape/target/debug/ JNIShape
# If needed to specify the version of java: javac -source 1.8 -target 1.8 JNIShape.java && ...

.PHONY: shape

javah:
	javac -h . JNIShape.java

lib:
	cd shape && cargo build
