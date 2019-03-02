class Jgrep < Formula
  desc "A command-line JSON processor in Rust. jgrep searches for selector patterns in json input, jgrep prints each json object that matches a pattern."
  homepage "https://github.com/gmmorris/jgrep"
  url "https://github.com/gmmorris/jgrep/releases/download/0.1.0.osx/jgrep"
  sha256 "5fca6b86b320bbd6204d65144cdd3756e6a9e72dd283a6c00827aebe87df9964"
  # depends_on "cmake" => :build

  def install
    # ENV.deparallelize  # if your formula fails when building in parallel
    # Remove unrecognized options if warned by configure
    system "./configure", "--disable-debug",
                          "--disable-dependency-tracking",
                          "--disable-silent-rules",
                          "--prefix=#{prefix}"
    # system "cmake", ".", *std_cmake_args
    system "make", "install" # if this fails, try separate make/make install steps
  end

  test do
    assert_equal "{\"name\":\"inigo montoya\"}\n", pipe_output("#{bin}/jgrep .name", '{"name":"inigo montoya"}')
  end
end