# Logger v0.1.0

Logger is a simple DLL that help you to debug your DLL usages or just log messages in a .log file

## How to use

### 1. Download the DLL
### 2. Add the DLL to your project
### 3. Add the DLL functions to your project (C# example):
```cs
using System.Runtime.InteropServices;

namespace TestRustDll
{
    [StructLayout(LayoutKind.Sequential)]
    struct LogOptions
    {
        public bool recreate;
        public string filepath;
    }

    class Program
    {
        [DllImport(@"E:\Dev\Rust\logger\target\debug\logger.dll")]
        public static extern void init_log(LogOptions options);

        [DllImport(@"E:\Dev\Rust\logger\target\debug\logger.dll")]
        public static extern void log_message(LogOptions options, string message);

        static void Main()
        {

        }
    }
}
```
### 4. Call the functions (C# example):
```cs
using System.Runtime.InteropServices;

namespace TestRustDll
{
    // LogOptions struct

    class Program
    {
        // DLL functions

        static void Main()
        {
            LogOptions options = new()
            {
                recreate = true,
                filepath = "debug.log"
            };

            init_log(options);

            log_message(options, "Hello from C#!");
        }
    }
}