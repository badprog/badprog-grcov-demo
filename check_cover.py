# https://github.com/badprog/badprog-grcov-demo

# import
import subprocess
import os
import shutil

# const var
PROJECT_NAME = "p1-arg-verifier"
REPORT_FORMAT = "html"
REPORT_DIR = f"target/coverage/{PROJECT_NAME}"
COVERAGE_FILE_DIR = "coverage-files"
PROFDATA_OUTPUT = "coverage.profdata"

# =============================================================================
# get_llvm_profdata_path
def get_llvm_profdata_path():
    """Get dynamically the absolute path for llvm-profdata."""
    print("\nSearching the path of the Rust toolchain...")
    
    command_target = ["rustc", "--print", "host-tuple"]
    command_to_run = ["rustc", "--print", "sysroot"]
    
    try:
        host_tuple = subprocess.run(
            command_target,
            check=True,
            capture_output=True,
            text=True
        ).stdout.strip()
        print(f"Detected host tuple: '{host_tuple}'")
    
    except:
        raise
    
    try:
        sysroot_output = subprocess.run(command_to_run,
                                        check=True, capture_output=True, text=True).stdout.strip()
        toolchain_path = os.path.join(sysroot_output, "lib", "rustlib")
        llvm_bin_path = os.path.join(
            toolchain_path, host_tuple, "bin", "llvm-profdata")
        if os.path.exists(llvm_bin_path):
            print(f"Path found: {llvm_bin_path}")
            return llvm_bin_path
        else:
            raise RuntimeError(f"'llvm-profdata' not found at the supposed path -> {llvm_bin_path}")
    except:
        raise

# =============================================================================
# run_command
def run_command(command, shell=False, env=None, check=True):
    """Execute a command ans display its status."""
    #
    if shell == True:
        if isinstance(command, list):
            # needs to tranform the list into an str
            command_to_execute = " ".join(command)
        else:
            # command is already an str
            command_to_execute = command
    else: # shell == False
        if isinstance(command, list):
            # here the command must not have special chars to manage (like '*')
            command_to_execute = command
        else:
             # isinstance(command, str)
            command_to_execute = command
        
    # 
    print(f"\nExecution: {' '.join(command)}")

    #
    try:
        subprocess.run(command_to_execute, check=check, shell=shell, env=env)
    except subprocess.CalledProcessError as e:
        print(f"❌ Error during command execution! Code: {e.returncode}")
        if e.stdout:
            print(f"Stdout:\n{e.stdout}")
        if e.stderr:
            print(f"Stderr:\n{e.stderr}")
        raise

# =============================================================================
# generate_coverage_report
def generate_coverage_report():
    """Generate the coverage report if every step is OK."""
    LLVM_PROFDATA_EXEC = get_llvm_profdata_path()

    # cleaning
    print("\nStep 1: Cleaning binaries and profile data...")
    if os.path.exists(REPORT_DIR):
        shutil.rmtree(REPORT_DIR) # rm report directory

    run_command(["cargo", "clean", "-p", PROJECT_NAME])

    if os.path.exists(COVERAGE_FILE_DIR):
        shutil.rmtree(COVERAGE_FILE_DIR) # rm profile files directory

    os.makedirs(COVERAGE_FILE_DIR, exist_ok=True) # re create the profile files directory
    os.makedirs(REPORT_DIR, exist_ok=True) # re create the report directory

    # Generate .profraw files and run tests
    print("\nStep 2: Executing instrumented tests...")
    run_command(["cargo",
                 "test",
                 "-p", PROJECT_NAME,
                 "--tests",
                 "--bins"])

    # Merge raw data (.profraw -> .profdata)
    print("\nStep 3: Merge profile data...")
    profdata_merge_command = [
        f"{LLVM_PROFDATA_EXEC}",
        "merge",
        "-sparse",
        f"{COVERAGE_FILE_DIR}/*.profraw",
        "-o",
        PROFDATA_OUTPUT
    ]
    run_command(profdata_merge_command, shell=True) # needs shell=True to replace the '*' char by real files

    # Generate report with grcov
    print("\nStep 4: Generate HTML report...")
    grcov_command = [
        "grcov",
        PROFDATA_OUTPUT,
        "--binary-path", "./target/",
        "-s", ".",
        "-t", REPORT_FORMAT,
        "-o", REPORT_DIR,
        "--keep-only", f"{PROJECT_NAME}/**",
        "--ignore", f"{PROJECT_NAME}/src/main.rs",
    ]
    run_command(grcov_command)
    print("\n✅ Report generated successfully.")
    print(
        f"HTML report available here: {os.path.join(REPORT_DIR, REPORT_FORMAT, 'index.html')}")

# =============================================================================
# __name__
if __name__ == "__main__":
    try:
        generate_coverage_report()
    except Exception as e:
        print(f"\n❌ Error: Coverage process failed -> {e}")
