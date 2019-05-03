class Jg < Formula
  version '0.1.4'
  desc "A command-line JSON processor in Rust. jg searches for selector patterns in json input, jg prints each json object that matches a pattern."
  homepage "https://github.com/gmmorris/jg"

  if OS.mac?
      url "https://github.com/gmmorris/jg/releases/download/#{version}/jg-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "af9edac7eec4168d26edeb812e5079bf27b3d14230a5acfb260ebf0ce3c973d3"
  elsif OS.linux?
      url "https://github.com/gmmorris/jg/releases/download/#{version}/jg-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "673d8567a1e5952bcd29e87ef711b74012b30babc0e74bbfbd04be8c622876d8"
  end

  def install
    bin.install "jg"
  end

  test do
    assert_equal "{\"name\":\"jeff goldblum\"}\n", pipe_output("#{bin}/jg .name", '{"name":"jeff goldblum"}')
  end
end