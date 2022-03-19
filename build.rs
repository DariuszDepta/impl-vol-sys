/*
 * MIT license
 *
 * Copyright (c) 2022 Dariusz Depta
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

fn main() {
  let output_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  cxx_build::bridge("src/lib.rs")
    .file("letsberational/rationalcubic.cpp")
    .file("letsberational/normaldistribution.cpp")
    .file("letsberational/lets_be_rational.cpp")
    .file("letsberational/erf_cody.cpp")
    .out_dir(output_dir.join("lib"))
    .flag_if_supported("-std=c++11")
    .compile("impl-vol");
}