import std.stdio : writefln, writeln;
import std.string : join;

void main(string[] args) {
    writeln("noxide: a Nox-built D environment diagnostic");
    writefln("arguments: %s", args.length - 1);
    writefln("payload: %s", args[1 .. $].join(" | "));
    writeln("backend: D Rider");
}
