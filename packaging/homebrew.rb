class Jgrep < Formula
  desc "A command-line JSON processor in Rust. jg searches for selector patterns in json input, jg prints each json object that matches a pattern."
  homepage "https://github.com/gmmorris/jg"
  url "https://github.com/gmmorris/jg/archive/0.1.1.osx.tar.gz"
  sha256 "3ae1dc0f831764eb5dfce7db5644a95f2d02e0ee423941d1b8deb9e60684fdc9"
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
    assert_equal "{\"name\":\"jeff goldblum\"}\n", pipe_output("#{bin}/jg .name", '{"name":"jeff goldblum"}')
  end
end