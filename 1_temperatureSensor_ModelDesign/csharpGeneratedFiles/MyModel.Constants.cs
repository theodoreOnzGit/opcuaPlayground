/* ========================================================================
 * Copyright (c) 2005-2021 The OPC Foundation, Inc. All rights reserved.
 *
 * OPC Foundation MIT License 1.00
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * The complete license agreement can be found here:
 * http://opcfoundation.org/License/MIT/1.00/
 * ======================================================================*/

using System;
using System.Collections.Generic;
using System.Text;
using System.Reflection;
using System.Xml;
using System.Runtime.Serialization;
using Opc.Ua;

namespace MyModel
{
    #region DataType Identifiers
    /// <summary>
    /// A class that declares constants for all DataTypes in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class DataTypes
    {
        /// <summary>
        /// The identifier for the temperature_reading DataType.
        /// </summary>
        public const uint temperature_reading = 1;
    }
    #endregion

    #region Method Identifiers
    /// <summary>
    /// A class that declares constants for all Methods in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class Methods
    {
        /// <summary>
        /// The identifier for the temperatureSensor_Read Method.
        /// </summary>
        public const uint temperatureSensor_Read = 3;
    }
    #endregion

    #region Object Identifiers
    /// <summary>
    /// A class that declares constants for all Objects in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class Objects
    {
        /// <summary>
        /// The identifier for the temperatureSensor1 Object.
        /// </summary>
        public const uint temperatureSensor1 = 6;

        /// <summary>
        /// The identifier for the temperature_reading_Encoding_DefaultBinary Object.
        /// </summary>
        public const uint temperature_reading_Encoding_DefaultBinary = 10;

        /// <summary>
        /// The identifier for the temperature_reading_Encoding_DefaultXml Object.
        /// </summary>
        public const uint temperature_reading_Encoding_DefaultXml = 18;

        /// <summary>
        /// The identifier for the temperature_reading_Encoding_DefaultJson Object.
        /// </summary>
        public const uint temperature_reading_Encoding_DefaultJson = 26;
    }
    #endregion

    #region ObjectType Identifiers
    /// <summary>
    /// A class that declares constants for all ObjectTypes in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class ObjectTypes
    {
        /// <summary>
        /// The identifier for the temperatureSensor ObjectType.
        /// </summary>
        public const uint temperatureSensor = 2;
    }
    #endregion

    #region Variable Identifiers
    /// <summary>
    /// A class that declares constants for all Variables in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class Variables
    {
        /// <summary>
        /// The identifier for the temperatureSensor_sensorName Variable.
        /// </summary>
        public const uint temperatureSensor_sensorName = 4;

        /// <summary>
        /// The identifier for the temperatureSensor_temperatureValueC Variable.
        /// </summary>
        public const uint temperatureSensor_temperatureValueC = 5;

        /// <summary>
        /// The identifier for the temperatureSensor1_sensorName Variable.
        /// </summary>
        public const uint temperatureSensor1_sensorName = 8;

        /// <summary>
        /// The identifier for the temperatureSensor1_temperatureValueC Variable.
        /// </summary>
        public const uint temperatureSensor1_temperatureValueC = 9;

        /// <summary>
        /// The identifier for the MyModel_BinarySchema Variable.
        /// </summary>
        public const uint MyModel_BinarySchema = 11;

        /// <summary>
        /// The identifier for the MyModel_BinarySchema_NamespaceUri Variable.
        /// </summary>
        public const uint MyModel_BinarySchema_NamespaceUri = 13;

        /// <summary>
        /// The identifier for the MyModel_BinarySchema_Deprecated Variable.
        /// </summary>
        public const uint MyModel_BinarySchema_Deprecated = 14;

        /// <summary>
        /// The identifier for the MyModel_BinarySchema_temperature_reading Variable.
        /// </summary>
        public const uint MyModel_BinarySchema_temperature_reading = 15;

        /// <summary>
        /// The identifier for the MyModel_XmlSchema Variable.
        /// </summary>
        public const uint MyModel_XmlSchema = 19;

        /// <summary>
        /// The identifier for the MyModel_XmlSchema_NamespaceUri Variable.
        /// </summary>
        public const uint MyModel_XmlSchema_NamespaceUri = 21;

        /// <summary>
        /// The identifier for the MyModel_XmlSchema_Deprecated Variable.
        /// </summary>
        public const uint MyModel_XmlSchema_Deprecated = 22;

        /// <summary>
        /// The identifier for the MyModel_XmlSchema_temperature_reading Variable.
        /// </summary>
        public const uint MyModel_XmlSchema_temperature_reading = 23;
    }
    #endregion

    #region DataType Node Identifiers
    /// <summary>
    /// A class that declares constants for all DataTypes in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class DataTypeIds
    {
        /// <summary>
        /// The identifier for the temperature_reading DataType.
        /// </summary>
        public static readonly ExpandedNodeId temperature_reading = new ExpandedNodeId(MyModel.DataTypes.temperature_reading, MyModel.Namespaces.MyModel);
    }
    #endregion

    #region Method Node Identifiers
    /// <summary>
    /// A class that declares constants for all Methods in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class MethodIds
    {
        /// <summary>
        /// The identifier for the temperatureSensor_Read Method.
        /// </summary>
        public static readonly ExpandedNodeId temperatureSensor_Read = new ExpandedNodeId(MyModel.Methods.temperatureSensor_Read, MyModel.Namespaces.MyModel);
    }
    #endregion

    #region Object Node Identifiers
    /// <summary>
    /// A class that declares constants for all Objects in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class ObjectIds
    {
        /// <summary>
        /// The identifier for the temperatureSensor1 Object.
        /// </summary>
        public static readonly ExpandedNodeId temperatureSensor1 = new ExpandedNodeId(MyModel.Objects.temperatureSensor1, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the temperature_reading_Encoding_DefaultBinary Object.
        /// </summary>
        public static readonly ExpandedNodeId temperature_reading_Encoding_DefaultBinary = new ExpandedNodeId(MyModel.Objects.temperature_reading_Encoding_DefaultBinary, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the temperature_reading_Encoding_DefaultXml Object.
        /// </summary>
        public static readonly ExpandedNodeId temperature_reading_Encoding_DefaultXml = new ExpandedNodeId(MyModel.Objects.temperature_reading_Encoding_DefaultXml, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the temperature_reading_Encoding_DefaultJson Object.
        /// </summary>
        public static readonly ExpandedNodeId temperature_reading_Encoding_DefaultJson = new ExpandedNodeId(MyModel.Objects.temperature_reading_Encoding_DefaultJson, MyModel.Namespaces.MyModel);
    }
    #endregion

    #region ObjectType Node Identifiers
    /// <summary>
    /// A class that declares constants for all ObjectTypes in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class ObjectTypeIds
    {
        /// <summary>
        /// The identifier for the temperatureSensor ObjectType.
        /// </summary>
        public static readonly ExpandedNodeId temperatureSensor = new ExpandedNodeId(MyModel.ObjectTypes.temperatureSensor, MyModel.Namespaces.MyModel);
    }
    #endregion

    #region Variable Node Identifiers
    /// <summary>
    /// A class that declares constants for all Variables in the Model Design.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class VariableIds
    {
        /// <summary>
        /// The identifier for the temperatureSensor_sensorName Variable.
        /// </summary>
        public static readonly ExpandedNodeId temperatureSensor_sensorName = new ExpandedNodeId(MyModel.Variables.temperatureSensor_sensorName, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the temperatureSensor_temperatureValueC Variable.
        /// </summary>
        public static readonly ExpandedNodeId temperatureSensor_temperatureValueC = new ExpandedNodeId(MyModel.Variables.temperatureSensor_temperatureValueC, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the temperatureSensor1_sensorName Variable.
        /// </summary>
        public static readonly ExpandedNodeId temperatureSensor1_sensorName = new ExpandedNodeId(MyModel.Variables.temperatureSensor1_sensorName, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the temperatureSensor1_temperatureValueC Variable.
        /// </summary>
        public static readonly ExpandedNodeId temperatureSensor1_temperatureValueC = new ExpandedNodeId(MyModel.Variables.temperatureSensor1_temperatureValueC, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the MyModel_BinarySchema Variable.
        /// </summary>
        public static readonly ExpandedNodeId MyModel_BinarySchema = new ExpandedNodeId(MyModel.Variables.MyModel_BinarySchema, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the MyModel_BinarySchema_NamespaceUri Variable.
        /// </summary>
        public static readonly ExpandedNodeId MyModel_BinarySchema_NamespaceUri = new ExpandedNodeId(MyModel.Variables.MyModel_BinarySchema_NamespaceUri, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the MyModel_BinarySchema_Deprecated Variable.
        /// </summary>
        public static readonly ExpandedNodeId MyModel_BinarySchema_Deprecated = new ExpandedNodeId(MyModel.Variables.MyModel_BinarySchema_Deprecated, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the MyModel_BinarySchema_temperature_reading Variable.
        /// </summary>
        public static readonly ExpandedNodeId MyModel_BinarySchema_temperature_reading = new ExpandedNodeId(MyModel.Variables.MyModel_BinarySchema_temperature_reading, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the MyModel_XmlSchema Variable.
        /// </summary>
        public static readonly ExpandedNodeId MyModel_XmlSchema = new ExpandedNodeId(MyModel.Variables.MyModel_XmlSchema, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the MyModel_XmlSchema_NamespaceUri Variable.
        /// </summary>
        public static readonly ExpandedNodeId MyModel_XmlSchema_NamespaceUri = new ExpandedNodeId(MyModel.Variables.MyModel_XmlSchema_NamespaceUri, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the MyModel_XmlSchema_Deprecated Variable.
        /// </summary>
        public static readonly ExpandedNodeId MyModel_XmlSchema_Deprecated = new ExpandedNodeId(MyModel.Variables.MyModel_XmlSchema_Deprecated, MyModel.Namespaces.MyModel);

        /// <summary>
        /// The identifier for the MyModel_XmlSchema_temperature_reading Variable.
        /// </summary>
        public static readonly ExpandedNodeId MyModel_XmlSchema_temperature_reading = new ExpandedNodeId(MyModel.Variables.MyModel_XmlSchema_temperature_reading, MyModel.Namespaces.MyModel);
    }
    #endregion

    #region BrowseName Declarations
    /// <summary>
    /// Declares all of the BrowseNames used in the Model Design.
    /// </summary>
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class BrowseNames
    {
        /// <summary>
        /// The BrowseName for the MyModel_BinarySchema component.
        /// </summary>
        public const string MyModel_BinarySchema = "MyModel";

        /// <summary>
        /// The BrowseName for the MyModel_XmlSchema component.
        /// </summary>
        public const string MyModel_XmlSchema = "MyModel";

        /// <summary>
        /// The BrowseName for the Read component.
        /// </summary>
        public const string Read = "Read";

        /// <summary>
        /// The BrowseName for the sensorName component.
        /// </summary>
        public const string sensorName = "sensorName";

        /// <summary>
        /// The BrowseName for the temperature_reading component.
        /// </summary>
        public const string temperature_reading = "temperature_reading";

        /// <summary>
        /// The BrowseName for the temperatureSensor component.
        /// </summary>
        public const string temperatureSensor = "temperatureSensor";

        /// <summary>
        /// The BrowseName for the temperatureSensor1 component.
        /// </summary>
        public const string temperatureSensor1 = "temperatureSensor1";

        /// <summary>
        /// The BrowseName for the temperatureValueC component.
        /// </summary>
        public const string temperatureValueC = "temperatureValueC";
    }
    #endregion

    #region Namespace Declarations
    /// <summary>
    /// Defines constants for all namespaces referenced by the model design.
    /// </summary>
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    public static partial class Namespaces
    {
        /// <summary>
        /// The URI for the MyModel namespace (.NET code namespace is 'MyModel').
        /// </summary>
        public const string MyModel = "http://www.opcfoundation.org/MyModel/";

        /// <summary>
        /// The URI for the OpcUa namespace (.NET code namespace is 'Opc.Ua').
        /// </summary>
        public const string OpcUa = "http://opcfoundation.org/UA/";

        /// <summary>
        /// The URI for the OpcUaXsd namespace (.NET code namespace is 'Opc.Ua').
        /// </summary>
        public const string OpcUaXsd = "http://opcfoundation.org/UA/2008/02/Types.xsd";
    }
    #endregion
}