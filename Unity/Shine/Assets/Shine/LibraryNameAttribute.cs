using System;

namespace Shine
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public class LibraryNameAttribute : Attribute
    {
        public string LibraryName { get; }

        public LibraryNameAttribute(string libraryName)
        {
            LibraryName = libraryName;
        }
    }
}